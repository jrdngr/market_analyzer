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

        const y = d3.scaleBand()
            .domain(data.prices.map(d => d.strike))
            .range([height - margin.bottom, margin.top]);

        const yAxis = g => g
            .attr("transform", `translate(${margin.left},0)`)
            .call(d3.axisLeft(y).ticks(10))
            .call(g => g.select(".domain").remove());

        const svg = d3.create("svg")
            .attr("viewBox", [0, 0, width, height]);

        svg.append("g")
            .selectAll("rect")
            .data(data.prices)
            .join("rect")
            .attr("x", margin.left)
            .attr("y", d => y(d.strike))
            .attr("height", y.bandwidth())
            .attr("width", width)
            .attr("fill", getColor);

        svg.append("g")
            .call(yAxis)
            .selectAll("text")
            .attr("fill", "white")
            .attr("font-size", "1em");

        if (data.quote.last >= data.minPrice && data.quote.last <= data.maxPrice) {
            svg.append("g")
            .selectAll("rect")
            .data([data.quote.last])
            .join("rect")
            .attr("class", "price")
            .attr("x", margin.left)
            .attr("y", d => y(nearestStrike(d)) - (y.bandwidth() / 3))
            .attr("height", y.bandwidth() / 3)
            .attr("width", width)
            .attr("fill", "steelblue");
        }

        el.append(svg.node());
    });

    function rgbaToHex(r, g, b, a) {
        return '#' + [r, g, b, a].map(x => {
            const hex = x.toString(16)
            return hex.length === 1 ? '0' + hex : hex
        }).join('')
    }

    function scaleAlpha(alpha) {
        return alpha;
    }

    function getColor(point) {
        let r = 0;
        let g = 0;
        let b = 0;
        let a;

        if (point.gamma_exposure > 0) {
            r = 255;
            a = point.gamma_exposure / data.maximum;
        } else {
            g = 255;
            a = Math.abs(point.gamma_exposure / data.minimum);
        }

        a = scaleAlpha(a);
        a = Math.floor(a * 255);

        return rgbaToHex(r, g, b, a);
    }

    function nearestStrike(price) {
        let nearest = null;
        let distance = null;

        for (const strike of data.prices.map(d => d.strike)) {
            let d = Math.abs(strike - price);
            if (!nearest || !distance) {
                nearest = strike;
                distance = d;
            } else if (d < distance) {
                distance = d;
                nearest = strike;
            }
        }

        console.log(nearest);

        return nearest;
    }

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
