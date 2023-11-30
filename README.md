[![GitHub stars](https://img.shields.io/github/stars/ItsHyde-dev/http-client-rust?style=social)](https://github.com/ItsHyde-dev/http-client-rust/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/ItsHyde-dev/http-client-rust)](https://github.com/ItsHyde-dev/http-client-rust/issues)


# 🚀 HTTP REST Client Built Using Rust

Welcome to my HTTP REST client project, built with Rust to streamline API testing and development processes. This powerful tool incorporates a full-fledged file parsing system and a user-friendly terminal UI for an intuitive user experience.

## ⚙️ Features

- 💻 **Terminal UI**: The terminal UI provides a seamless and user-friendly interface for making and managing HTTP requests.
  
- 📂 **Custom File Parsing System**: The custom file parsing system allows for efficient handling of data, enhancing the versatility of the client.

## 📚 Getting Started

To use the system, follow these steps:

1. **Clone the Repository**: Clone the repository to your local machine.

```
git clone https://github.com/ItsHyde-dev/http-client-rust.git
```

2. **Install Dependencies**: Navigate to the project directory and use Cargo to install the client.

```
cd http-client-rust
cargo install --path ./
```

3. **Load Requests**: To load requests into the system, use the following command with the path to your request file.
```
http-client -p <filepath>
```

## 📋 File Format

The HTTP REST client project supports a custom file format for specifying requests. This structure allows for easy organization and customization of HTTP requests. Below is an example of the file format:

```plaintext
########
{
  name: Sample GET Request
  url: https://api.example.com/data
  method: GET
}

######
{
  name: Sample POST Request
  url: https://api.example.com/create
  method: POST
  headers:
  {
    "Content-Type": "application/json"
  },
  body:
  {
    "key": "value"
  }
}

#####
{
  name: Sample PUT Request
  url: https://api.example.com/update
  method: PUT
  headers:
  {
    "Authorization": "Bearer YOUR_ACCESS_TOKEN"
  },
  body:
  {
    "updated_key": "updated_value"
  }
}
```

### Note:

- For the `headers` and `body` fields, ensure that the provided JSON is valid, containing key-value pairs.
- The opening and closing curly brackets for the JSON in both `headers` and `body` should be on a new line for proper formatting.
- The keys for the format should not be enclosed in "". (This is due to personal taste) 

  Example:

  ```plaintext
  headers:
  {
    "Content-Type": "application/json"
  },
  body:
  {
    "key": "value"
  }


## 🔮 Future Scope

Moving forward, I plan to enable the system to send multipart form data with file attachments, further enhancing its capabilities for handling various types of requests.

I'm excited to continue evolving this project and welcome any feedback or collaboration opportunities.
