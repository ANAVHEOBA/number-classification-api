# Number Classification API

A REST API service that provides mathematical properties and fun facts about numbers. Built with Rust and Actix-web framework.

## 🚀 Features

- Get mathematical properties of a number (prime, perfect, Armstrong)
- Determine if a number is odd or even
- Calculate digit sum
- Fetch interesting mathematical facts about numbers
- CORS enabled
- JSON responses
- Error handling for invalid inputs

## 🛠️ Technologies

- Rust
- Actix-web framework
- Serde for JSON serialization
- Reqwest for HTTP client
- Numbers API for fun facts

## 📝 API Specification

### Endpoint


GET /api/classify-number?number={number}


### Success Response (200 OK)


json
{
"number": 371,
"is_prime": false,
"is_perfect": false,
"properties": ["armstrong", "odd"],
"digit_sum": 11,
"fun_fact": "371 is an Armstrong number because 3^3 + 7^3 + 1^3 = 371"
}


### Error Response (400 Bad Request)


json
{
"number": "alphabet",
"error": true
}


## 🚀 Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Installation

1. Clone the repository

bash
git clone https://github.com/anavheoba/number-classification-api.git
cd number-classification-api



2. Build the project

bash
cargo build --release



3. Run the server

bash
cargo run



The API will be available at `http://localhost:8080`

## 🔧 Usage Examples

Using curl:

bash
Get properties of number 371
curl "http://localhost:8080/api/classify-number?number=371"




## 🌐 Deployment

The API is deployed at: `[https://number-classification-api-7.onrender.com/]`

## ⚡ Performance

- Average response time: < 500ms
- Handles concurrent requests efficiently
- CORS enabled for cross-origin access

## 🧪 Testing

Run the test suite:


bash
cargo test



## 📝 API Properties Combinations

The `properties` field in the response can have the following combinations:
- `["armstrong", "odd"]` - for Armstrong numbers that are odd
- `["armstrong", "even"]` - for Armstrong numbers that are even
- `["odd"]` - for non-Armstrong numbers that are odd
- `["even"]` - for non-Armstrong numbers that are even

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## 👤 Author

Your Name
- GitHub: [@anavheoba](https://github.com/anavheoba)

## 🙏 Acknowledgments

- [Numbers API](http://numbersapi.com) for providing number facts
- Actix-web framework
- Rust community


curl -i "https://number-classification-api-12.onrender.com/api/classify-number?number=371"