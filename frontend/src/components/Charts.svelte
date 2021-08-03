<script>
    import ChartRow from './ChartRow.svelte';

    let rows = [];
    let nextId = 0;

    addRow();

    function addRow() {
        nextId += 1;
        let row = {
            id: nextId,
        };
        rows = [...rows, row];
        console.log(rows);
    }

    function deleteRow(id) {
        for (let i = 0; i < rows.length; i++) {
            if (rows[i].id == id) {
                rows.splice(i, 1);
            }
        }
        rows = [ ...rows ];
    }
</script>

<div class="rows">
{#each rows as row}
    <div class="row">
        <button on:click={() => deleteRow(row.id)}>x</button>
        <ChartRow></ChartRow>
    </div>
{/each}
</div>
<button on:click={addRow}>New Row</button>

<style>
</style>
