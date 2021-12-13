![made-with-rust](img/made-with-rust.svg?style=centerme)
![built-with-love](img/built-with-love.svg?style=centerme)
![works-on-linux](img/works-on-linux.svg?style=centerme)
![works-on-my-machine](img/works-on-my-machine.svg?style=centerme)
![60-of-the-time-works-every-time](img/60-of-the-time-works-every-time.svg?style=centerme)

# ChatBox

## 🚩 Summary
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

## 🥡 Project Structure
> - **The project is divided into two files. Clients and Server. The client as well as the server has a module, where the functions are located.**
![Tree](img/tree.png?style=centerme)

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

## Serveur
> - **We start the server with the command (cargo run).**<br>
![Serveur](img/serveur.png?style=centerme)
> - **When the server is launched, the ONLINE and REGISTERED tables are created. If the REGISTERED table already exists, it is not created. Each message, registration and modification log in the database is saved in /log/sql.log /log/info.log log/credentials.log log/chat.log**<br>

## Client
> - **We start the client with the command (cargo run).**<br>
![Client](img/client.png?style=centerme)
> - **When launching the client, the main menu opens. You choose the first choice to register. (If you are already registered, you can directly choose the second choice).**<br>
![Menu](img/menu.png?style=centerme)
>> - **Register**<br>
![Inscription](img/inscription.png?style=centerme)<br></br>
>> - **If the user is not already registered in the database, a QR Code is generated for two-factor identification.**<br>
![Inscription-QRCode](img/inscription_qrcode.png?style=centerme)<br></br>
>> - ⛔ **In case the user is already registered, the program reports the error and cuts the connection.**<br>
![Inscription-Erreur](img/inscription_erreur.png?style=centerme)<br></br>
> - **Once registered, you can log in and choose choice n°2.**
> - **Login**.<br>
![Connexion](img/connexion.png?style=centerme)<br></br>
> - **OTP entry for 2FA authentication**.<br>
![OTP](img/otp.png?style=centerme)<br></br>
>> - **In case the entered identifiers are correct and verified in the database, the connection is successful.**<br>
![Connexion_Successful](img/connexion_successful.png?style=centerme)<br></br>
>> - ⛔ **In case the entered credentials are not correct (Password or OTP), the connection fails and the TCP link is cut.**<br>
![Connexion_Error](img/connexion_error.png?style=centerme)<br></br>
> - **Once you have authenticated, you can chat and enjoy a secure discussion.**<br>
![Chat](img/chat.png?style=centerme)<br></br>