use reqwest::Client;

pub struct NumberService {
    http_client: Client,
}

impl NumberService {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }

    pub fn is_prime(&self, n: i64) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    pub fn is_perfect(&self, n: i64) -> bool {
        if n <= 1 {
            return false;
        }
        let mut sum = 1;
        let sqrt = (n as f64).sqrt() as i64;
        
        for i in 2..=sqrt {
            if n % i == 0 {
                sum += i;
                if i != n / i {
                    sum += n / i;
                }
            }
        }
        sum == n
    }

    pub fn is_armstrong(&self, n: i64) -> bool {
        let digits: Vec<i64> = n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        let power = digits.len() as u32;
        
        let sum: i64 = digits.iter()
            .map(|&d| d.pow(power))
            .sum();
        
        sum == n
    }

    pub fn digit_sum(&self, n: i64) -> i64 {
        n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .sum()
    }

    pub async fn get_fun_fact(&self, n: i64) -> Result<String, reqwest::Error> {
        let url = format!("http://numbersapi.com/{}/math", n);
        let response = self.http_client.get(&url)
            .header("Content-Type", "text/plain")
            .timeout(std::time::Duration::from_secs(5))  // Add 5 second timeout
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    
    pub fn get_properties(&self, n: i64) -> Vec<String> {
        let mut properties = Vec::new();
        
        if self.is_armstrong(n) {
            properties.push("armstrong".to_string());
        }
        
        if n % 2 == 0 {
            properties.push("even".to_string());
        } else {
            properties.push("odd".to_string());
        }
        
        properties
    }
}