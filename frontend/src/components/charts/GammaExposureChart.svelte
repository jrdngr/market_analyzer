<script>
	import { beforeUpdate } from 'svelte';
	import * as d3 from 'd3';
	
    export let data;
	
	let el = document.createElement("div");

	beforeUpdate(() => {
        const margin = ({top: 20, right: 0, bottom: 70, left: 70})
        const width = 800;
        const height = 500;

        el.textContent = "";

        d3.select(el).style("background", "black");

        const x = d3.scaleBand()
            .domain(data.map(d => d.strike))
            .rangeRound([margin.left, width - margin.right])
            .padding(0.2);

        const y = d3.scaleLinear()
            .domain([0, d3.max(data, d => Math.abs(d.gammaExposure))])
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
            .attr("y", d => y(Math.abs(d.gammaExposure)))
            .attr("height", d => y(0) - y(Math.abs(d.gammaExposure)))
            .attr("width", x.bandwidth())
            .attr("fill", d => d.gammaExposure >= 0 ? "tomato" : "steelblue");

        svg.append("g")
            .call(xAxis)
            .selectAll("text")
            .data(data)
            .attr("transform", "translate(12,25) rotate(90)")
            .attr("fill", d => d.gammaExposure === 0 ? "transparent" : "white")
            .attr("font-size", "1em");

        svg.append("g")
            .call(yAxis)
            .selectAll("text")
            .attr("fill", "white")
            .attr("font-size", "1em");

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
