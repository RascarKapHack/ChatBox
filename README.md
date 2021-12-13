![made-with-rust](img/made-with-rust.svg?style=centerme)
![built-with-love](img/built-with-love.svg?style=centerme)
![works-on-linux](img/works-on-linux.svg?style=centerme)
![works-on-my-machine](img/works-on-my-machine.svg?style=centerme)
![60-of-the-time-works-every-time](img/60-of-the-time-works-every-time.svg?style=centerme)

# ChatBox

## 🚩 Summary
<div style="text-align: justify"> your-text-here </div>
This chatbox developed in Rust, uses the TCP/IP protocol to make several clients communicate with each other via a remote server. Exchanges are encrypted via a dynamic key, and a registration of the user is necessary to be able to speak on the chat. When registering, a QR Code is sent to the client's terminal. When connecting, the user must enter his password and the otp code generated via the QR Code. The server checks the integrity of the information in its database, and authorizes or not the connection. The server records each log of the actions performed on the database side, the chat history, as well as the established and failed connections.

## 📦 Dependencies
```bash
apt install sqlite3
```

## 🔐 Encryption & Encode & Security
- Dynamic encryption AES256
- QR Code generation for Two factor authentification (2FA)
- Storage of password hashes (SHA256) in the database
- Base64 & Base32 Encode

## 🎨 Database (SQLITE3)
### .table ONLINE
| Client_Id |        ip:port        | statut  |
|-----------|-----------------------|---------|
|  htag     |  192.168.1.63:38520   | online  |
|  Client2  |  192.168.1.123:48150  | anonyme |

### .table REGISTERED
| User |                           password (SHA256)                              |      ip:port (last)     |    statut   |  online  |
|------|--------------------------------------------------------------------------|-------------------------|-------------|----------|
| leo  |     cc11410fc57ad8c7fd50839e7e97499a7d4de2e5cf6ac432ea848bbf6bcd1a67     |   192.168.1.63:38600    |     ko      |    ko    |
| htag |     2652875ee631c6fee36e7ebee192e8bdcdf54566f3c380e7bd3feb2adbc879e4     |   192.168.1.63:38520    |     ok      |    ok    |

## ✅ Usage
> serveur
>>```bash
>>cargo run
>>```
> client
>>```bash
>>cargo run
>>```


# 📕 Manual

## Client
> - la
> - ici
## Serveur
> - ici
> - là