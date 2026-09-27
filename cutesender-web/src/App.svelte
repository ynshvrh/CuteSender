<script lang="ts">
  let email = $state('');
  let loading = $state(false);
  let status = $state<'idle' | 'success' | 'error'>('idle');
  let errorMessage = $state('');

  // Relative API endpoint handled via reverse proxy
  const RUST_API_URL = '/api/send';

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!email || !email.includes('@')) {
      status = 'error';
      errorMessage = 'Будь ласка, вкажи правильну пошту ✨';
      return;
    }

    loading = true;
    status = 'idle';
    errorMessage = '';

    try {
      const response = await fetch(RUST_API_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: email.trim() }),
      });

      const data = await response.json();

      if (response.ok) {
        status = 'success';
        email = '';
      } else {
        status = 'error';
        errorMessage = data.error || 'Щось пішло не так при відправці';
      }
    } catch (err) {
      status = 'error';
      errorMessage = 'Не вдалося зʼєднатися з сервером. Спробуйте пізніше.';
    } finally {
      loading = false;
    }
  }
</script>

<main class="container">
  <div class="card">
    <div class="header">
      <span class="badge">Nyaw</span>
      <h1>Cutesender</h1>
      <p class="subtitle">Введи пошту, щоб отримати тепле повідомлення</p>
    </div>

    <form onsubmit={handleSubmit}>
      <div class="input-group">
        <input
          type="email"
          placeholder="your_email@gmail.com"
          bind:value={email}
          disabled={loading}
          required
        />
      </div>

      <button type="submit" disabled={loading} class="btn-send">
        {#if loading}
          <span class="spinner"></span> Відправляємо промінчики...
        {:else}
          Надіслати трішки тепла 🐾
        {/if}
      </button>
    </form>

    {#if status === 'success'}
      <div class="banner success">
        <p>✨ Лист уже в дорозі! Заглянь у скриньку)  💖</p>
      </div>
    {/if}

    {#if status === 'error'}
      <div class="banner error">
        <p>😿 {errorMessage}</p>
      </div>
    {/if}
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: #fdf5f5;
    color: #432828;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
  }

  .container {
    width: 100%;
    max-width: 440px;
    padding: 20px;
    box-sizing: border-box;
  }

  .card {
    background: #ffffff;
    border-radius: 24px;
    padding: 36px 28px;
    box-shadow: 0 10px 30px rgba(226, 125, 142, 0.12);
    border: 1px solid #fae1e4;
    text-align: center;
  }

  .badge {
    background: #ffe5ec;
    color: #c94a6d;
    font-size: 12px;
    font-weight: 600;
    padding: 4px 10px;
    border-radius: 20px;
    display: inline-block;
    margin-bottom: 12px;
  }

  h1 {
    font-size: 26px;
    margin: 0 0 6px 0;
    color: #2e1d1d;
  }

  .subtitle {
    font-size: 14px;
    color: #8c7272;
    margin: 0 0 24px 0;
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  input {
    width: 100%;
    padding: 14px 16px;
    border: 1.5px solid #f3d4d8;
    border-radius: 14px;
    font-size: 15px;
    outline: none;
    box-sizing: border-box;
    transition: all 0.2s;
    background: #fffafa;
  }

  input:focus {
    border-color: #e27d8e;
    background: #ffffff;
    box-shadow: 0 0 0 4px rgba(226, 125, 142, 0.15);
  }

  .btn-send {
    background: linear-gradient(135deg, #e27d8e, #d46277);
    color: white;
    border: none;
    padding: 14px 20px;
    border-radius: 14px;
    font-size: 15px;
    font-weight: 600;
    cursor: pointer;
    transition: transform 0.15s, opacity 0.2s, box-shadow 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    box-shadow: 0 4px 14px rgba(226, 125, 142, 0.3);
  }

  .btn-send:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 6px 18px rgba(226, 125, 142, 0.4);
  }

  .btn-send:active:not(:disabled) {
    transform: translateY(0);
  }

  .btn-send:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .banner {
    margin-top: 18px;
    padding: 12px 16px;
    border-radius: 12px;
    font-size: 14px;
    animation: fadeIn 0.3s ease;
  }

  .banner p {
    margin: 0;
  }

  .success {
    background: #eaf8ee;
    color: #236c3b;
    border: 1px solid #ccecd4;
  }

  .error {
    background: #fdf0f0;
    color: #a82e2e;
    border: 1px solid #fad2d2;
  }

  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid #ffffff;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
    display: inline-block;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(4px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
