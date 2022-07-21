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
