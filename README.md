
 📖 Pasaka Swahili Bible API


This is a blazing-fast, Swahili Bible API built for developers who need scripture access in milliseconds. Whether you're working on a mobile app, website, or chatbot, the Pasaka API gets you Bible content faster than you can say Amen i hope so 😅😅😅.



🌟 Features

- 🔥 Ultra-fast RESTful API built with **Rust** and **Axum**
- 📚 Access to **all books**, **chapters**, **verses** from the Swahili Bible
- 🔍 Powerful **search** by keyword
- 🔐 API Key secured endpoints
- 🧾 Swagger UI documentation
- 🖥️ Hosted & ready to use



 🧪 Test the API (Swagger)

You can explore all endpoints via Swagger:

🔗 [Swagger UI Documentation](https://pasaka.4insec.com/swagger-ui)

To authorize:

1. Click the "Authorize" button
2. Enter your API key in this format:

```
x-api-key: your_api_key_here
```


🧰 Using the API via `curl`

If you prefer CLI or scripting, here's a sample:

```bash
curl -H "x-api-key: pasaka_api_7a782fcd-da06-455xxxxxx" \
https://pasaka.4insec.com/books/66/1/1
```

✅ **Response**:

```json
{
  "data": {
    "verse_number": "1",
    "verse_text": "Ufunuo wa Yesu Kristo, aliopewa na Mungu awaonyeshe watumwa wake mambo ambayo kwamba hayana budi kuwako upesi..."
  },
  "success": true
}
```



 📦 Endpoints

| Method | Endpoint                    | Description                    |
|--------|-----------------------------|--------------------------------|
| GET    | `/books`                    | Get Entire Bible           |
| GET    | `/books/{book_id}`          | Get info about a specific book |
| GET    | `/books/{book_id}/{chapter}`| Get a chapter from a book      |
| GET    | `/books/{book_id}/{chapter}/{verse}` | Get a specific verse  |
| GET    | `/search?query={text}`      | Search verses by keyword       |
| POST   | `/auth/register`            | Register new user              |
| POST   | `/auth/login`               | Login & get token              |



🔐 Authentication

All Bible data routes are protected by API Key.

 Add header in requests:

```http
x-api-key: your_api_key
```

You’ll receive your API key upon registration (or request one manually).



🙏 Credits

We give full credit to the following amazing contributors who provided clean, usable Swahili Bible JSON data:

- 💡 [Shemm Junior](https://github.com/shemmjunior) – For his clean JSON work on Swahili Bible structure.
- 💡 [Kalebu Gwalugano](https://github.com/Kalebu) – For contributing and maintaining publicly available Swahili Bible content.

Without their groundwork, this API wouldn't be possible.



👨‍💻 Tech Stack

- ⚙️ **Rust**
- 🚀 **Axum**
- 🔐 **API Key Security**
- 📘 **Swagger + Utoipa**
- 🧪 **Tests included**
- 🔁 **Fast JSON search**


 💌 Want to Contribute?

We welcome PRs, ideas, and contributions!

1. Fork this repo
2. Clone and build with `cargo run`
3. Add your changes
4. Open a pull request



🪪 [Licence](./LICENCE)

This project is open-source and available under the **Jesus License**.



> Made with ❤️ to spread the Word quickly.
