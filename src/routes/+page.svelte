<script lang="ts">
  import { markedHighlight } from "marked-highlight";
  import { invoke } from "@tauri-apps/api/core";
  import hljs from "highlight.js";
  import { Marked } from "marked";

  import 'highlight.js/styles/github-dark.css';

  const marked = new Marked(
    markedHighlight({
      emptyLangClass: 'hljs',
      langPrefix: 'hljs language-',
      highlight(code, lang, info) {
        const language = hljs.getLanguage(lang) ? lang : 'plaintext';
        return hljs.highlight(code, { language }).value;
      }
    })
  );

  let content = $state("");
  let markdown = $derived(marked.parse(content));

  // async function greet(event: Event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }
</script>

<main>
  <textarea bind:value={content}></textarea>
  <article class="prose">{@html markdown}</article>
</main>
