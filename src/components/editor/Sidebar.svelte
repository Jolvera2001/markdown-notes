<script lang="ts">
    let {
        documents,
        onDocumentSelected,
    }: {
        documents: Document[] | null;
        onDocumentSelected: (index: number) => void;
    } = $props();
</script>

<aside class="w-64 h-screen border-1 border-gray-400 bg-gray-300">
    <nav class="p-4">
        {#if documents == null}
            <p>No documents loaded</p>
        {:else}
            {#each documents as doc, i}
                <button
                    class="w-full flex items-center gap-2 px-1 border-sm hover:bg-gray-400 text-left"
                    onclick={() => onDocumentSelected(i)}
                >
                    {doc.name}
                </button>
            {/each}
        {/if}
    </nav>
</aside>
