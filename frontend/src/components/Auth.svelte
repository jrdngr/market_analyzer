<script>
    import { navigate } from "svelte-routing";
    import { login } from '../common/apis/td';

    const urlParams = new URLSearchParams(window.location.search);
    const code = decodeURI(urlParams.get(`code`));
    login(code);
    navigate("/", { replace: true })
</script>

<p>Auth!</p>

<a href="https://localhost:5000">Back</a>
