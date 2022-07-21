pub fn get_logo(distro_id: &str) -> &str {
    match distro_id {
        // arch linux logo
        "arch" => {
            r"   _____                .__     
  /  _  \_______   ____ |  |__  
 /  /_\  \_  __ \_/ ___\|  |  \ 
/    |    \  | \/\  \___|   Y  \
\____|__  /__|    \___  >___|  /
        \/            \/     \/ "
        }

        "ubuntu" => {
            r" ____ ______.                 __         
|    |   \_ |__  __ __  _____/  |_ __ __ 
|    |   /| __ \|  |  \/    \   __\  |  \
|    |  / | \_\ \  |  /   |  \  | |  |  /
|______/  |___  /____/|___|  /__| |____/ 
              \/           \/            "
        }

        "debian" => {
            r"________        ___.   .__               
\______ \   ____\_ |__ |__|____    ____  
 |    |  \_/ __ \| __ \|  \__  \  /    \ 
 |    `   \  ___/| \_\ \  |/ __ \|   |  \
/_______  /\___  >___  /__(____  /___|  /
        \/     \/    \/        \/     \/ "
        }

        // default logo
        _ => {
            r".____    .__                     
|    |   |__| ____  __ _____  ___
|    |   |  |/    \|  |  \  \/  /
|    |___|  |   |  \  |  />    < 
|_______ \__|___|  /____//__/\_ \
        \/       \/            \/"
        }
    }
}
