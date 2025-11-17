<script lang="ts">
    let {
        documents,
        onDocumentSelected,
    }: {
        documents: Document[] | null;
        onDocumentSelected: (index: number) => void;
    } = $props();
</script>

<div>
    {#if documents == null}
        <p>No documents loaded</p>
    {:else}
        {#each documents as doc, i}
            <button onclick={() => onDocumentSelected(i)}>{doc.name}</button>
        {/each}
    {/if}
</div>
