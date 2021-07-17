<script>
    import { storeCode, login } from '../common/td';

    const urlParams = new URLSearchParams(window.location.search);
    const code = decodeURI(urlParams.get(`code`));
    storeCode(code);
    const refreshToken = login();
    console.log(refreshToken);
    // window.location.href = "https://localhost:5000";
</script>

<p>Auth!</p>

<a href="https://localhost:5000">Back</a>
