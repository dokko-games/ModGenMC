<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";
  import type { Version } from '../../../static/types';
    import { goto } from "$app/navigation";

  let name = "";
  let path = "";
  let versions: Version[] = [];
  let selectedVersion = "";

  // get versions from Rust on mount
  onMount(async () => {
    versions = await invoke("get_available_mc_versions");
  });

  async function selectFolder() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select Project Folder",
    });
    if (typeof selected === "string") {
      path = selected;
    }
  }

  function createProject() {
    invoke("create_project", { name, path, selectedVersion });
  }

  function goBack() {
    goto('/');
  }
</script>

<div class="create-page">
  <h1>
  <span style="float:right; cursor:pointer;" on:click={goBack}>×</span>
  Create Project
</h1>

<div class="form-group">
  <label>Project Name</label>
  <input placeholder="Enter project name" bind:value={name} />
</div>

<div class="form-group">
  <label>Project Path</label>
  <div class="path-select">
    <span class="path-display">{path || "No folder selected"}</span>
    <button on:click={selectFolder}>Choose…</button>
  </div>
</div>

<div class="form-group">
  <label>Version</label>
  <select bind:value={selectedVersion}>
    <option disabled value="">Select version</option>
    {#each versions as ver}
      <option value={ver.major}.{ver.minor}.{ver.patch}>
        Fabric {ver.major}.{ver.minor}{ver.patch > 0 ? "." + ver.patch : ""}
      </option>
    {/each}
  </select>
</div>

<button on:click={createProject}>Create</button>
</div>

<style>
.create-page {
  margin: 20px;
}
h1 {
  position: relative;
  font-size: 1.8em;
}
h1 span {
  font-size: 1.5em;
}
.form-group {
  margin: 16px 0;
  display: flex;
  flex-direction: column;
}
label {
  font-weight: bold;
  margin-bottom: 6px;
}
input {
  padding: 8px;
  border-radius: 4px;
  border: 1px solid #444;
  background: #111;
  color: #eee;
}
.path-select {
  display: flex;
  gap: 8px;
  align-items: center;
}
.path-display {
  flex: 1;
  padding: 8px;
  background: #111;
  border: 1px solid #444;
  border-radius: 4px;
  color: #ccc;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
select {
  padding: 8px;
  border-radius: 4px;
  background: #111;
  color: #eee;
  border: 1px solid #444;
}
button {
  padding: 10px 18px;
  background: #333;
  color: #eee;
  border: none;
  border-radius: 4px;
}
button:hover {
  background: #444;
}
</style>
