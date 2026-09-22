const template = (
  <html>
    <head>
      <title>🔥 Blazing 🔥</title>
      <meta charset="UTF-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <link rel="stylesheet" href="/styles.css" />
      <script>{`var data = [${d}]`}</script>
      <Assets />
    </head>
    <body>
      <header>
        <h1>Welcome to the Jungle</h1>
      </header>
      <App />
      <footer>The Bottom</footer>
    </body>
  </html>
);
