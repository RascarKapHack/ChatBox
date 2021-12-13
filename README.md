![made-with-rust](img/made-with-rust.svg?style=centerme)
![built-with-love](img/built-with-love.svg?style=centerme)
![works-on-linux](img/works-on-linux.svg?style=centerme)
![works-on-my-machine](img/works-on-my-machine.svg?style=centerme)
![60-of-the-time-works-every-time](img/60-of-the-time-works-every-time.svg?style=centerme)

# ChatBox

## 📦 Dependencies
```bash
apt install sqlite3
```

## 🔐 Encryption & Sécurity
- Dynamic encryption AES256
- QR Code generation for Two factor authentification (2FA)
- Storage of password hashes (SHA256) in the database

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