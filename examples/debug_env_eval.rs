use orion_variate::{EnvChecker, EnvDict, EnvEvaluable, ValueType};

fn main() {
    let json_content = r#"{
    "database_url": "${DB_URL}",
    "api_key": "${API_KEY}",
    "timeout": 30
}"#;

    println!("原始内容：");
    println!("{}", json_content);
    println!("\n");

    let mut env_dict = EnvDict::new();
    env_dict.insert("DB_URL", ValueType::from("postgresql://localhost/mydb"));
    env_dict.insert("API_KEY", ValueType::from("secret-key-123"));

    println!("环境变量字典：");
    println!("  DB_URL = postgresql://localhost/mydb");
    println!("  API_KEY = secret-key-123");
    println!("\n");

    let evaluated = json_content.to_string().env_eval(&env_dict);

    println!("env_eval 之后：");
    println!("{}", evaluated);
    println!("\n");

    println!("needs_env_eval: {}", evaluated.needs_env_eval());
    if evaluated.needs_env_eval() {
        println!("未定义的变量: {:?}", evaluated.list_env_vars());
    }
}
