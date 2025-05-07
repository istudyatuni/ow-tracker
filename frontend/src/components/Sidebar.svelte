<script module>
  import CloseIcon from "~icons/tabler/x";
  import MenuIcon from "~icons/tabler/menu-2";
  import LanguageIcon from "~icons/tabler/language-hiragana";
  import GithubIcon from "~icons/tabler/brand-github";

  import FileUpload from "./atoms/FileUpload.svelte";
  import ShiplogCategories from "./ShiplogCategories.svelte";

  import { LANGUAGE_NAMES, save_language } from "../lib/language";
  import { LANGUAGE, SETTINGS } from "../lib/stores";
  import { t } from "../lib/i18n";
</script>

<script>
  let opened = $state(false);
  function handle_select_lang(e) {
    save_language(e.target.value);
    window.location.reload();
  }
</script>

<div class="bar above-map" class:border={opened}>
  <button onclick={() => (opened = !opened)}>
    {#if opened}
      <CloseIcon />
    {:else}
      <MenuIcon />
    {/if}
  </button>

  <div class:hidden={!opened}>
    <FileUpload />

    <div class="block-wrapper categories">
      <ShiplogCategories />
    </div>

    <div class="block-wrapper">
      <label>
        <input
          type="checkbox"
          name="show-spoilers"
          class="other"
          bind:checked={$SETTINGS.hide_spoilers} />
        {$t("hide-spoilers-checkbox")}
      </label>
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
  .block-wrapper.categories {
    max-width: 35em;
  }
  .brand-icon {
    color: white;
  }
  /*.border {
    border: 1px $color solid;
  }*/
  button,
  select {
    cursor: pointer;
    margin-bottom: 5px;
  }
</style>
