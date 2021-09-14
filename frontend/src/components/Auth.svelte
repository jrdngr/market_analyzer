<script>
    import { login } from '../common/apis/td';
    const urlParams = new URLSearchParams(window.location.search);
    const code = decodeURI(urlParams.get(`code`));
    login(code);
</script>

<p>Auth!</p>

<a href="https://localhost:5000">Back</a>