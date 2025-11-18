<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog"

  import Sidebar from "../components/editor/Sidebar.svelte";
  import Editor from "../components/editor/Editor.svelte";

  let docs = $state<Document[] | null>(null);
  let selectedDocument = $state<Document | null>(null);
  $inspect(docs)

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
</script>

<main>
  <div class="flex divide-x-1">
    <button onclick={load_files}>Load Markdown</button>
    <Sidebar documents={docs} onDocumentSelected={select_document}/>
    <Editor document={selectedDocument} />
  </div>
</main>
