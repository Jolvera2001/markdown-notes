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
    let parsed = $derived(marked.parse(document?.content ?? ''));
</script>

<div class="h-full w-full flex">
    {#if document}
        <div>
            <textarea bind:value={document.content}></textarea>
        </div>
    {:else}
        <div>no document loaded</div>
    {/if}
    <div>
        <article>{@html parsed}</article>
    </div>
</div>
