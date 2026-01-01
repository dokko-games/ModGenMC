<script>
    import Sidebar from "./Sidebar.svelte";
    import RecentProjects from "./RecentProjects.svelte";
</script>

<div class="layout">
    <Sidebar/>
    <RecentProjects/>
</div>

<style>
.layout {
  display: flex;
  height:100vh
}
</style>