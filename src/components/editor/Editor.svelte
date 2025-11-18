<script lang="ts">
    import { markedHighlight } from "marked-highlight";
    import { Marked } from "marked";
    import hljs from "highlight.js";
    import "highlight.js/styles/github-dark.css";

    const marked = new Marked(
        markedHighlight({
            emptyLangClass: "hljs",
            langPrefix: "hljs language-",
            highlight(code, lang, info) {
                const language = hljs.getLanguage(lang) ? lang : "plaintext";
                return hljs.highlight(code, { language }).value;
            },
        }),
    );

    let { document }: { document: Document | null } = $props();
    let parsed = $derived(marked.parse(document?.content ?? ""));
</script>

<div class="h-full w-full flex p-6">
    {#if document}
        <div class="w-1/2">
            <textarea class="w-full h-full" bind:value={document.content}
            ></textarea>
        </div>
    {:else}
        <div>no document loaded</div>
    {/if}
    <div class="w-1/2  p-2 prose">
        <article>{@html parsed}</article>
    </div>
</div>
