# haemolacriaa 
My personal website written in rust with Leptos and Actix 

# Deployment via VPS (my way) 

## Setup the webserver

Begin by purchasing a VPS with debian installed, 
then a domain, and add DNS records via your registrar 

Then create a ssh key for your domain, and ssh into your server 
```
ssh-keygen
ssh-copy-id root@example.com
ssh root@example.com
```

Next, set some sshd conf settings to disable password auth, 
and use only the ssh key
```
echo 'PasswordAuthentication no\nChallengeResponseAuthentication no\nUsePAM no' >> /etc/ssh/sshd_config
```
If you lose your key, login via the terminal on your VPS's website 
and then change these settings back so you can generate a new key 
and run this command again 

Next, we want to install Nginx and certbot so we can use it 
as a reverse proxy for multiple web apps, and have secure https connections for each one
```
apt-get install nginx python3-certbot-nginx
```

By default, the firewall might block connections to HTTP and HTTPS ports, allow them like so:
```
ufw allow 80
ufw allow 443
ufw allow 'Nginx Full'
```

Now, we are going to turn off the nginx service to generate our https certificate.
Replace \<example.com\> with your domain:
```
systemctl stop nginx
certbot certonly --standalone --register-unsafely-without-email -d <example.com>
```

To enable auto renew, we will use a cron job:
```
crontab -e
```

This will open a file in your favorite editor (obviously vim), just append this line:
```
0 0 1 * * certbot --nginx renew
```

Now we need to setup our nginx config file! 
This is an example nginx config.
Replace each occurrence of \<example.com\> with your domain, and write this to /etc/nginx/nginx.conf:
```
events {
        worker_connections 1024;
}

http {
        server_tokens off;
        add_header Content-Security-Policy "default-src 'self';" always;
        add_header X-Content-Type-Options "nosniff" always;
        proxy_hide_header X-Runtime;

        server {
                server_name www.<example.com>;
                return 301 $scheme://<example.com>$request_uri;
        }

        server {
                listen 80;
                listen [::]:80;
                server_name <example.com>;

                return 301 https://$host$request_uri;
        }

        server {
                listen [::]:443 ssl;
                listen 443 ssl;

                ssl_certificate "/etc/letsencrypt/live/<example.com>/fullchain.pem";
                ssl_certificate_key "/etc/letsencrypt/live/<example.com>/privkey.pem";
                ssl_trusted_certificate "/etc/letsencrypt/live/<example.com>/chain.pem";

                server_name <example.com>;

                location / {
                        proxy_pass http://127.0.0.1:3000;
                        proxy_set_header Host $host;
                }

                error_page 404 /404;
        }
}
```

Copy the haemolacriaa.service file to /etc/systemd/system/
```
cp haemolacriaa.service /etc/systemd/system/haemolacriaa.service
```

Build this project and move the files to /app:
```
cargo leptos build --release
cp target/release/haemolacriaa /app/haemolacriaa
cp -r target/site /app/site
mkdir /app/site/assets
mv /app/site/*.webp /app/site/assets/.
```

Now we can start nginx, along with this web app:
```
systemctl start nginx
systemctl enable haemolacriaa
systemctl start haemolacriaa
```

You might notice that our haemolacriaa service failed to start, and that's because it requires a database connection on the same host.
Which is what we'll set up next.

## Setup postgresql

Install postgresql on the VPS:
```
apt-get install postgresql
```

Change to the postgres user, and open psql
```
su - postgres
psql
```

The first thing you should do is change the default password for the user postgres. Replace \<password-here\> with a password of your choice:
```
alter user postgres with encrypted password '<password-here>';
```

Add a user and our new database:
```
create user songuser;
create database songdb;
```

Set the new user's password and give database permissions:
```
alter user songuser with encrypted password '<password-here>';
grant all privileges on database songdb to songuser;
```

Then set our new database's owner to our new user (important)
```
alter database songdb owner to songuser;
```

Now we need to edit the config to listen on our VPS's static ipv4 
Use this command to find the directory of the config files, and exit:
```
SHOW config_file;
\q
```

Open up the config file in your editor (vim), currently my config is located in
/etc/postgresql/15/main/
```
vim /etc/postgresql/15/main/postgresql.conf
```

Navigate (in vim) to the line by using /listen_address, or append this line 
to the file. Replace \<your-server-ip\> with the ipv4 address of your VPS and quit:
```
listen_addresses = 'localhost,<your-server-ip>'
:wq
```

Now we need to allow our dev machine to connect to the DB if we want to run tests and edit the web app.
Append this line to pg_hba.conf, replace \<your-ip\> with your personal ip address:
```
vim /etc/postgresql/15/main/pg_hba.conf
hostssl   songdb             songuser             <your-ip>/0        scram-sha-256
:wq
```

Note that your machine likely doesn't have a static IP. If your IP changes, 
you will have to update \<your-ip\> before connecting again or calculate a proper subnet mask (the number beyond the trailing slash).

Then, exit the shell and restart postgresql, allowing traffic to the port of the DB.
```
exit
systemctl restart postgresql
ufw allow 5432
```

Test the connection on your own machine, exit the ssh connection to the VPS and do:
```
psql postgres://songuser:<your-songuser-password>@<your-domain>:5432/songdb
\q
```

So far, there should be no errors. To connect the web server to the
database, you will need to run it with the following environment variables:
```
PG_USER=songuser
PG_PASSWORD=<your-songuser-password>
PG_HOST=127.0.0.1
PG_PORT=5432
PG_DATABASE=songdb
```

\<your-songuser-password\> is the password you gave the user "songuser" earlier. I recommend that you set these via the service file that we set up previously.
Finally, restart the haemolacriaa service. Everything should be working.
```
systemctl restart haemolacriaa
```
