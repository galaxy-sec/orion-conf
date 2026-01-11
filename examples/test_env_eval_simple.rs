use orion_variate::{EnvChecker, EnvDict, EnvEvaluable, ValueType};

fn main() {
    println!("=== 测试 1: 简单字符串 ===");
    let simple = "Hello ${NAME}!".to_string();
    let mut env_dict = EnvDict::new();
    env_dict.insert("NAME", ValueType::from("World"));
    let result = simple.env_eval(&env_dict);
    println!("输入: Hello ${{NAME}}!");
    println!("输出: {}", result);
    println!();

    println!("=== 测试 2: 包含冒号的值 ===");
    let url_test = "url: ${URL}".to_string();
    env_dict.insert("URL", ValueType::from("http://localhost"));
    let result2 = url_test.env_eval(&env_dict);
    println!("输入: url: ${{URL}}");
    println!("输出: {}", result2);
    println!();

    println!("=== 测试 3: 多行 JSON ===");
    let json = r#"{"url":"${URL}"}"#.to_string();
    let result3 = json.env_eval(&env_dict);
    println!("输入: {{\"url\":\"${{URL}}\"}}");
    println!("输出: {}", result3);
    println!();

    println!("=== 测试 4: 包含斜杠的值 ===");
    let path_test = "path: ${PATH}".to_string();
    env_dict.insert("PATH", ValueType::from("postgresql://localhost/mydb"));
    let result4 = path_test.env_eval(&env_dict);
    println!("输入: path: ${{PATH}}");
    println!("输出: {}", result4);
}
