<script module>
  import CloseIcon from "~icons/tabler/x";
  import MenuIcon from "~icons/tabler/menu-2";
  import LanguageIcon from "~icons/tabler/language-hiragana";
  import GithubIcon from "~icons/tabler/brand-github";

  import ConsiderIgnored from "@/components/atoms/ConsiderIgnored.svelte";
  import FileUpload from "@/components/atoms/FileUpload.svelte";
  import HideSpoilers from "@/components/atoms/HideSpoilers.svelte";
  import ShiplogCategories from "@/components/ShiplogCategories.svelte";
  import SidebarApply from "@/components/atoms/SidebarApply.svelte";
  import ShowUnexplored from "@/components/atoms/ShowUnexplored.svelte";

  import { LANGUAGE_NAMES, save_language } from "@/lib/language";
  import {
    LANGUAGE,
    reset_selected_categories,
    SAVE_FOUND,
    SESSION_SETTINGS,
  } from "@/lib/stores";
  import { t } from "@/lib/i18n";
</script>

<script>
  let opened = $state(false);
  let changed = $state(false);

  function handle_select_lang(e) {
    save_language(e.target.value);
    window.location.reload();
  }
  function show_full_map() {
    window.location.hash = "";
    window.location.reload();
  }
  function handle_file_upload() {
    reset_selected_categories();
    window.location.reload();
  }
</script>

<div class="bar above-map">
  <button onclick={() => (opened = !opened)}>
    {#if opened}
      <CloseIcon />
    {:else}
      <MenuIcon />
    {/if}
  </button>

  <div class:hidden={!opened}>
    <div class:hidden={!$SESSION_SETTINGS.welcome_popup_done}>
      <FileUpload upload={handle_file_upload} />

      <div class="block-wrapper categories">
        <ShiplogCategories />
      </div>

      <div class="block-wrapper">
        <HideSpoilers />
        {#if $SAVE_FOUND}
          <br />
          <ConsiderIgnored onchange={() => (changed = true)} />
          <br />
          <ShowUnexplored />
          <br />
          <SidebarApply disabled={!changed} />
          <button onclick={show_full_map}
            >{$t("sidebar-show-full-map-button")}</button>
        {/if}
      </div>
    </div>

    <select onchange={handle_select_lang}>
      {#each Object.entries(LANGUAGE_NAMES) as [key, name]}
        <option value={key} selected={$LANGUAGE === key}>{name}</option>
      {/each}
    </select>
    <span class="icon">
      <LanguageIcon width="25" height="25" />
    </span>
    <br />

    <a href="https://github.com/istudyatuni/ow-tracker" class="brand-icon"
      ><GithubIcon width="35" height="35" /></a>
  </div>
</div>

<style lang="scss">
  $color: #3280ff;

  .bar {
    position: absolute;
    top: 0;
    left: 0;

    height: auto;
    padding: 1em 1em;

    color: white;
  }
  .icon {
    padding-top: 5px;
    vertical-align: middle;
    margin-left: 4px;
  }
  .bar :global(.block-wrapper) {
    background-color: var(--bg);
    border-radius: 10px;
    padding: 10px 10px;
    margin-bottom: 5px;
    max-width: 25em;
  }
  .block-wrapper {
    &.categories {
      max-width: 35em;
    }
    & > button {
      margin-top: 10px;
    }
  }
  .brand-icon {
    color: white;
  }
  button,
  select {
    cursor: pointer;
    margin-bottom: 5px;
  }
</style>
