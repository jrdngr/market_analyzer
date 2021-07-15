<script>
	import { onMount } from 'svelte';
	import * as d3 from 'd3';
	
    export let data;
	
	let el;

	onMount(() => {
        let d = [];
        for (const [key, value] of Object.entries(data)) {
            d.push(value);
        }
		d3.select(el)
			.selectAll("div")
			.data(d)
			.enter()
			.append("div")
			.style("width", function(d) {
				return d + "px";
			})
			.text(function(d) {
				return d;
			});
    });
</script>

<main>
    <div bind:this={el} class="chart"></div>
</main>

<style>
    .chart :global(div) {
		font: 10px sans-serif;
		background-color: steelblue;
		text-align: right;
		padding: 3px;
		margin: 1px;
		color: white;
	}
</style>
