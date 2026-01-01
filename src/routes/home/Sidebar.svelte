<script>
    import { goto } from "$app/navigation";
    import { getName, getVersion } from "@tauri-apps/api/app";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    let appName = "";
    let appVersion = "";
    onMount(async () => {
        appName = await getName();
        appVersion = await getVersion();
    });
    function handleCreate() {
      goto('/create');
    }
</script>
<div class="sidebar">
    <h1 class="appname">
        {appName && appVersion ? `${appName} v${appVersion}` : "Loading..."}
      </h1>
    <button class="glass" on:click={handleCreate}>Create...</button>
    <button class="glass">Open...</button>
    <button class="glass">Import...</button>
</div>
<style>
button.glass {
    height: 50px;
    font-family: 'Poppins';
    font-weight: 50px;
    background-color: rgba(209, 209, 209, 0.151);
    font-size: 15px;
    color: white;
    backdrop-filter: blur(5px);
    border-radius: 9px;
    border: none;
    cursor: pointer;
    box-shadow: 10px 10px 10px rgba(0,0,0,0.6);
    transition: 0.3s ease-in-out;
    width: 98%;
    text-shadow: 2px 2px 2px black;
}
button.glass:hover {
    width: 100%;
    background-color: rgba(255, 255, 255, 0.201);
}
.sidebar {
  width: 40%;
  min-width: 311px;
  max-width: 600px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 15px;
  user-select: none;
  background-color: rgb(46, 46, 46);
  background-image: url("/assets/art/sidebar_bg.png");
  background-position: center;
  background-repeat: no-repeat;

  position: relative;
  overflow: hidden;
}
.appname {
  margin: "0 0 12px 0";
  color: white;
  text-shadow: 2px 2px 13px black;
  cursor: default;
  user-select: none;
}
</style>