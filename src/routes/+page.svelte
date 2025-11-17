<script lang="ts">
  import { markedHighlight } from "marked-highlight";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog"
  import { Marked } from "marked";
  import hljs from "highlight.js";

  import Sidebar from "../components/editor/Sidebar.svelte";

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

  let docs = $state<Document[] | null>(null);
  let selectedDocument = $state<Document | null>(null);

  function select_document(index: number) {
    if (docs) {
      selectedDocument = docs[index];
    }
  }

  async function load_files() {
    try {
      const path = await open({
        multiple: false,
        directory: true,
      });
      docs = await invoke<Document[]>("load_files", { directoryPath: path });
      console.log(docs)
    } catch (error) {
      console.log("something went wrong", error);
    }
  }

  // async function greet(event: Event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }
</script>

<main>
  <Sidebar documents={docs} />
  <!-- <button onclick={load_files}>Load Markdown</button>
  <textarea bind:value={content}></textarea>
  <article class="prose">{@html markdown}</article> -->
</main>
