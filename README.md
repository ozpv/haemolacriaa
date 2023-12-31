# haemolacriaa
my personal website, written in *rust*  
you can preview an older, current version at [https://haemolacriaa.com/](https://haemolacriaa.com/)  

## how to configure
config settings for frontend are located in **frontend/src/config.rs**
  
# deploy section will be re-written soon!
  
<s>## how to deploy
Clone the repository and cd  
```
git clone https://github.com/ozpv/haemolacriaa.git
cd haemolacriaa
```

Install rust (from [rustup](https://rustup.rs/)) 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
  
Install WebAssembly target  
```
rustup target add wasm32-unknown-unknown
```

### if rustup doesn't work in zsh...
add this to your *.zshenv* file in $HOME:  
```
export PATH="$HOME/.cargo/bin/"
```  
and restart your shell by signing out.  

Update/install tailwindcss  
```
chmod +x update-tailwind
./update-tailwind
```

run it locally with:  
```
trunk serve
```
then visit 127.0.0.1:8080 in your browser  
this can be configured in Trunk.toml (see [configuration](https://trunkrs.dev/configuration/))  
or, deploy the contents located in **dist/** to a server:  
```
trunk build  
cd dist  
scp -r ./ user@example.com:/path/to/www/.  
```  
you probably have a better way, which automatically deploys it.</s>
