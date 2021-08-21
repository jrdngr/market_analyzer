<script>
	import { beforeUpdate } from 'svelte';
	import * as d3 from 'd3';
    import { randomId } from '../../common/utils';
	
    export let data;

	let el = document.createElement("div");
    const componentId = randomId();

	beforeUpdate(() => {
        const margin = ({top: 20, right: 0, bottom: 70, left: 70})
        const width = 800;
        const height = 500;

        const [minPrice, maxPrice] = d3.extent(data.prices, d => d.strike);
        const prices = data.prices.filter(p => p.strike >= minPrice && p.strike <= maxPrice);

        el.textContent = "";

        d3.select(el).style("background", "black");

        const startDate = new Date(data.startDate);
        const endDate = new Date(data.endDate);

        const x = d3.scaleTime()
            .domain([startDate, endDate])
            .range([margin.left, width - margin.right]);

        const xAxis = g => g
            .attr("transform", `translate(0,${height - margin.bottom})`)
            .call(d3.axisBottom(x).tickSizeOuter(0));

        const y = d3.scaleLinear()
            .domain([minPrice, maxPrice])
            .rangeRound([height - margin.bottom, margin.top])
            .clamp(true);

        const yAxis = g => g
            .attr("transform", `translate(${margin.left},0)`)
            .call(d3.axisLeft(y))
            .call(g => g.select(".domain").remove());

        const svg = d3.create("svg")
            .attr("viewBox", [0, 0, width, height]);

                
        /*
         * Draw axes
         */
         svg.append("g")
            .call(xAxis)
            .selectAll("text")
            .data(data)
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

        if (data.showGradient) {
            /* 
            * Draw background
            */
            const gradient = svg
                .append("linearGradient")
                .attr("id", `exposure-gradient-${componentId}`)
                .attr("x1", "0%")
                .attr("x2", "0%")
                .attr("y1", "100%")
                .attr("y2", "0%");
            
            const gradientOffset = (point) => {
                const strike = point.strike;
                
                const position = y(strike) - 0.5;
                const [min, max] = y.range();
                
                const strikeRatio = (position - min) / (max - min);
                return strikeRatio;
            };

            gradient
                .selectAll("stop")
                .data(prices)
                .join("stop")
                .attr("offset", gradientOffset)
                .style("stop-color", getColor);

            svg.append("rect")
                .attr("fill", `url(#exposure-gradient-${componentId})`)
                .attr("x", x.range()[0])
                .attr("y", y.range()[1])
                .attr("width", x.range()[1] - x.range()[0])
                .attr("height", y.range()[0] - y.range()[1]);
        }

        /* 
         * Draw lines with the gradient stop color at each strike price
         */
        if (data.highlightStrikes) {

            svg.append("g")
                .selectAll("rect")
                .data(prices)
                .join("rect")
                .attr("x", margin.left)
                .attr("y", d => y(d.strike) - 0.5)
                .attr("height", 1)
                .attr("width", width)
                .attr("fill", getColor);
        }

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
            .attr("y", d => y(Math.max(d.open, d.close)) - 0.5)
            .attr("width", 3)
            .attr("height", d => Math.abs(y(d.open) - y(d.close)))
            .attr("fill", "white");

        svg.append("g")
            .selectAll("rect")
            .data(data.ohlc)
            .join("rect")
            .attr("x", d => x(new Date(d.time)) + 1)
            .attr("y", d => y(d.high) - 0.5)
            .attr("width", 1)
            .attr("height", d => Math.abs(y(d.high) - y(d.low)))
            .attr("fill", "white");

        el.append(svg.node());
    });

    function rgbaToHex(r, g, b, a) {
        return '#' + [r, g, b, a].map(x => {
            const hex = x.toString(16)
            return hex.length === 1 ? '0' + hex : hex
        }).join('')
    }

    function getAlpha(point) {
        let a;

        if (point.gammaExposure > 0) {
            a = point.gammaExposure / data.maximum;
        } else {
            a = Math.abs(point.gammaExposure / data.minimum);
        }

        const brightness = data.brightness / 100;
        a = Math.min(a + brightness, 1.0);
        a = Math.max(a, 0.0);
        a = Math.floor(a * 255);

        return a;
    }

    function getColor(point) {
        let r = 0;
        let g = 0;
        let b = 0;
        let a = getAlpha(point);

        if (point.gammaExposure > 0) {
            r = 255;
        } else {
            g = 255;
        }

        if (data.flipColors) {
            let temp = r;
            r = g;
            g = temp;
        }

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
