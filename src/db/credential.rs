use mongodb::options;

pub struct Admin {
    username: String,
    pwd: String,
    auth_db: String,
}

#[allow(dead_code)]
impl Admin {
    pub fn new(username: String, pwd: String, auth_db: String) -> Self {
        return {
            Admin {
                username,
                pwd,
                auth_db,
            }
        };
    }
    pub fn init_cred(self) -> options::Credential {
        let crd_opt = options::Credential::builder()
            .username(self.username)
            .password(self.pwd)
            .source(self.auth_db)
            .build();
        return crd_opt;
    }
    
}
