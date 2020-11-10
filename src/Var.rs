pub fn create_users_table_string() -> String {
    "CREATE TABLE inv_users(\
     id INT(6) UNSIGNED AUTO_INCREMENT PRIMARY KEY,\
     username VARCHAR(32),\
     password VARCHAR(1024),\
     token VARCHAR(32),\
     root TINYINT(1),\
     mail VARCHAR(128),\
     displayname VARCHAR(32),\
     register_date DATETIME,\
     status VARCHAR(16)\
     );".to_string()
}