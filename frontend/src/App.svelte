<script>
    import { Router, Route } from "svelte-routing";
    import Charts from './components/Charts.svelte';

    export let url = "";
</script>

<Router url="{url}">
    <div>
        <Route path="/" component="{Charts}" />
    </div>
</Router>

<style>
</style>
