# haemolacriaa 
My personal website written in rust with Leptos and Actix 

# Deployment via VPS (my way) 

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
as a reverse proxy for our web apps, and have https connections to them
```
apt-get install nginx python3-certbot-nginx
```

Then tell the firewall to allow nginx
```
ufw allow 80
ufw allow 443
ufw allow 'Nginx Full'
```

Now, we are going to turn off the nginx service and generate our https certificate
```
systemctl stop nginx
certbot certonly --standalone --register-unsafely-without-email -d example.com
```

To enable auto renew, we will use a cron job:
```
crontab -e
```

this will open in your favorite editor, just append this line:
```
0 0 1 * * certbot --nginx renew
```

Now we need to setup our nginx config file! 
This is an example nginx config, write this to /etc/nginx/nginx.conf:
```
events {
        worker_connections 1024;
}

http {
        server {
                server_name www.example.com;
                return 301 https://example.com$request_uri;
        }

        server {
                listen 80;
                listen [::]:80;
                server_name example.com;

                return 301 https://$host$request_uri;
        }

        server {
                listen [::]:443 ssl;
                listen 443 ssl;

                ssl_certificate "/etc/letsencrypt/live/example.com/fullchain.pem";
                ssl_certificate_key "/etc/letsencrypt/live/example.com/privkey.pem";
                ssl_trusted_certificate "/etc/letsencrypt/live/example.com/chain.pem";

                server_name example.com;

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
