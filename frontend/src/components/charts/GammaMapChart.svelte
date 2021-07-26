<script>
	import { beforeUpdate } from 'svelte';
	import * as d3 from 'd3';
	
    export let data;

	let el = document.createElement("div");
    
	beforeUpdate(() => {
        const margin = ({top: 20, right: 0, bottom: 70, left: 70})
        const width = 800;
        const height = 500;

        const [minPrice, maxPrice] = d3.extent(data.prices, d => d.strike);

        el.textContent = "";

        d3.select(el).style("background", "black");

        const startDate = new Date();
        startDate.setDate(startDate.getDate() - 1);
        const endDate = new Date();

        const x = d3.scaleTime()
            .domain([startDate, endDate])
            .range([margin.left, width - margin.right]);

        const xAxis = g => g
            .attr("transform", `translate(0,${height - margin.bottom})`)
            .call(d3.axisBottom(x).tickSizeOuter(0));

        const y = d3.scaleLinear()
            .domain([minPrice, maxPrice])
            .rangeRound([height - margin.bottom, margin.top]);

        const yAxis = g => g
            .attr("transform", `translate(${margin.left},0)`)
            .call(d3.axisLeft(y))
            .call(g => g.select(".domain").remove());

        const svg = d3.create("svg")
            .attr("viewBox", [0, 0, width, height]);

        /* 
         * Draw background
         */

        svg.append("g")
            .selectAll("rect")
            .data(data.prices)
            .join("rect")
            .attr("x", margin.left)
            .attr("y", d => y(d.strike) - 0.5)
            .attr("height", 1)
            .attr("width", width)
            .attr("fill", getColor);

        svg.append("g")
            .call(xAxis)
            .selectAll("text")
            .data(data)
            .attr("transform", "translate(12,25) rotate(90)")
            .attr("fill", "white")
            .attr("font-size", "1em");
        
        svg.append("g")
            .call(yAxis)
            .selectAll("text")
            .attr("fill", "white")
            .attr("font-size", "1em");

        const yPrice = d3.scaleLinear()
            .domain([minPrice, maxPrice])
            .range([height - margin.bottom, margin.top]);

        /* 
         * Draw current price
         */
        svg.append("g")
            .selectAll("rect")
            .data([data.quote.last])
            .join("rect")
            .attr("class", "price")
            .attr("x", margin.left)
            .attr("y", d => yPrice(d) - 0.5)
            .attr("height", 1)
            .attr("width", width)
            .attr("fill", "yellow");

        /* 
         * Draw price chart
         */
        svg.append("g")
            .selectAll("rect")
            .data(data.ohlc)
            .join("rect")
            .attr("x", d => x(new Date(d.time)))
            .attr("y", d => y(Math.min(d.open, d.close)))
            .attr("width", 3)
            .attr("height", d => Math.abs(y(d.open) - y(d.close)))
            .attr("fill", "steelblue");

        svg.append("g")
            .selectAll("rect")
            .data(data.ohlc)
            .join("rect")
            .attr("x", d => x(new Date(d.time)))
            .attr("y", d => y(d.low))
            .attr("width", 1)
            .attr("height", d => Math.abs(y(d.high) - y(d.low)))
            .attr("fill", "steelblue");


        el.append(svg.node());
    });

    function rgbaToHex(r, g, b, a) {
        return '#' + [r, g, b, a].map(x => {
            const hex = x.toString(16)
            return hex.length === 1 ? '0' + hex : hex
        }).join('')
    }

    function scaleAlpha(alpha) {
        const brightness = data.brightness / 100;
        return Math.min(alpha + brightness, 1.0);
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
