<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Waytranslate</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            background-color: #0f172a;
            color: #e2e8f0;
            margin: 0;
            padding: 40px;
            line-height: 1.6;
        }
        h1, h2, h3 {
            color: #38bdf8;
        }
        code {
            background-color: #1e293b;
            padding: 4px 6px;
            border-radius: 6px;
            font-family: monospace;
            color: #facc15;
        }
        pre {
            background-color: #1e293b;
            padding: 16px;
            border-radius: 10px;
            overflow-x: auto;
        }
        .container {
            max-width: 900px;
            margin: auto;
        }
        .badge {
            display: inline-block;
            background: #334155;
            padding: 4px 10px;
            border-radius: 999px;
            margin-right: 8px;
            font-size: 12px;
        }
        .section {
            margin-bottom: 40px;
        }
        hr {
            border: none;
            border-top: 1px solid #334155;
            margin: 40px 0;
        }
    </style>
</head>
<body>
<div class="container">

<h1>Waytranslate</h1>

<p>
Offline translation CLI in Rust using LibreTranslate via Docker.
</p>

<div>
    <span class="badge">Rust</span>
    <span class="badge">Docker</span>
    <span class="badge">Offline</span>
    <span class="badge">Smart Fallback</span>
</div>

<hr>

<div class="section">
<h2>Overview</h2>
<p>
Waytranslate is a command-line tool written in Rust that utilizes a local LibreTranslate server running on Docker.
</p>

<p>
It supports:
</p>

<ul>
<li>Offline translation</li>
<li>Multi-language support</li>
<li>Automatic Latin fallback</li>
<li>No dependency on external APIs</li>
</ul>
</div>

<hr>

<div class="section">
<h2>Backend Installation (LibreTranslate)</h2>

<h3>1. Enable Docker</h3>

<pre><code>sudo systemctl enable docker
sudo systemctl start docker
sudo usermod -aG docker $USER</code></pre>

<p>Restart your session after adding the user to the docker group.</p>

<h3>2. Run the container</h3>

<pre><code>docker run -d \
  -p 5000:5000 \
  -e LT_LOAD_ONLY=en,pt,la \
  -e LT_DISABLE_RATE_LIMIT=true \
  --name libretranslate \
  libretranslate/libretranslate</code></pre>

<h3>3. Test the API</h3>

<pre><code>curl http://127.0.0.1:5000/languages</code></pre>

</div>

<hr>

<div class="section">
<h2>Automatic Latin Fallback</h2>

<p>
Some translation pairs do not exist directly (e.g., la → pt).
</p>

<p>
Waytranslate implements automatic fallback:
</p>

<pre><code>la → en → pt</code></pre>

<h3>Implemented Logic:</h3>

<pre><code>if source == "la" && target == "pt" {
    translate la → en
    translate en → pt
} else {
    normal translation
}</code></pre>

This ensures compatibility even when the direct model is unavailable.
</div>

<hr>

<div class="section">
<h2>Compilation</h2>

<pre><code>cargo clean
cargo build --release</code></pre>

Run:

<pre><code>./target/release/waytranslate</code></pre>

</div>

<hr>

<div class="section">
<h2>Usage Example</h2>

<pre><code>Salve mundi → Hello world</code></pre>

</div>

<hr>

<div class="section">
<h2>Architecture</h2>

<ul>
<li>Rust (CLI)</li>
<li>Reqwest (HTTP client)</li>
<li>Serde (JSON)</li>
<li>Docker (LibreTranslate Server)</li>
</ul>
</div>

<hr>

<div class="section">
<h2>Status</h2>

<p>
✔ Functional offline translation<br>
✔ Latin support<br>
✔ Automatic fallback<br>
✔ No 429 errors<br>
</p>
</div>

<hr>

<p>
Developed as a learning project and a practical tool for local translation.
</p>

</div>
</body>
</html>
