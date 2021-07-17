<script>
	import { beforeUpdate } from 'svelte';
	import * as d3 from 'd3';
	
    export let data;
	
	let el = document.createElement("div");

	beforeUpdate(() => {
        const margin = ({top: 20, right: 0, bottom: 30, left: 40})
        const width = 1280;
        const height = 800;

        el.textContent = "";

        d3.select(el).style("background", "white");

        const x = d3.scaleBand()
            .domain(data.map(d => d.strike))
            .rangeRound([margin.left, width - margin.right])
            .padding(0.1);

        const y = d3.scaleLinear()
            .domain([0, d3.max(data, d => Math.abs(d.gamma_exposure))])
            .range([height - margin.bottom, margin.top]);

        const xAxis = g => g
            .attr("transform", `translate(0,${height - margin.bottom})`)
            .call(d3.axisBottom(x).tickSizeOuter(0));

        const yAxis = g => g
            .attr("transform", `translate(${margin.left},0)`)
            .call(d3.axisLeft(y).ticks(10))
            .call(g => g.select(".domain").remove());

        const svg = d3.create("svg")
            .attr("viewBox", [0, 0, width, height]);

        svg.append("g")
            .selectAll("rect")
            .data(data)
            .join("rect")
            .attr("x", d => x(d.strike))
            .attr("y", d => y(Math.abs(d.gamma_exposure)))
            .attr("height", d => y(0) - y(Math.abs(d.gamma_exposure)))
            .attr("width", x.bandwidth())
            .attr("fill", d => d.gamma_exposure >= 0 ? "steelblue" : "tomato");

        svg.append("g")
            .call(xAxis)
            .selectAll("text")
            .attr("transform", "translate(15,25) rotate(90)");

        svg.append("g")
            .call(yAxis);

        el.append(svg.node());
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
		color: black;
	}
</style>
