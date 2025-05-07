<script module>
  import CloseIcon from "~icons/tabler/x";
  import MenuIcon from "~icons/tabler/menu-2";
  import LanguageIcon from "~icons/tabler/language-hiragana";
  import GithubIcon from "~icons/tabler/brand-github";

  import ShiplogCategories from "./ShiplogCategories.svelte";

  import { LANGUAGE_NAMES, save_language } from "../lib/language";
  import {
    export_save_to_browser_url,
    get_save_opened_facts,
  } from "../lib/saves";
  import { LANGUAGE, SETTINGS } from "../lib/stores";
  import { t } from "../lib/i18n";

  // todo: more
  const SAVE_LOCATIONS = [
    [
      "Steam, Windows",
      "%USERPROFILE%/AppData/LocalLow/Mobius Digital/Outer Wilds/SteamSaves",
    ],
    [
      "Steam, Linux",
      "$HOME/.local/share/Steam/steamapps/compatdata/753640/pfx/drive_c/users/steamuser/AppData/LocalLow/Mobius Digital/Outer Wilds/SteamSaves",
    ],
  ];
</script>

<script>
  let opened = $state(false);
  let file_upload_help_opened = $state(false);
  let input;

  function on_file_upload_click() {
    input.click();
  }
  async function handle_file_change(e) {
    let file = await e.target.files[0].text();
    let data = JSON.parse(file).shipLogFactSaves;
    export_save_to_browser_url(Object.keys(data), get_save_opened_facts(data));
    window.location.reload();
  }
  function handle_select_lang(e) {
    save_language(e.target.value);
    window.location.reload();
  }
</script>

<div class="bar above-map" class:border={opened}>
  <button
    onclick={() => {
      if (opened) {
        file_upload_help_opened = false;
      }
      opened = !opened;
    }}>
    {#if opened}
      <CloseIcon />
    {:else}
      <MenuIcon />
    {/if}
  </button>

  <div class:hidden={!opened}>
    <button type="button" onclick={on_file_upload_click}
      >{$t("upload-save-file-button")}</button>
    <button
      type="button"
      class="question-button"
      onclick={() => (file_upload_help_opened = !file_upload_help_opened)}
      >?</button>
    <input
      bind:this={input}
      id="fileinput"
      type="file"
      accept=".owsave"
      class="hidden"
      onchange={handle_file_change} />
    <br />

    {#if file_upload_help_opened}
      <div class="block-wrapper">
        <h3>{$t("file-upload-help-header")}</h3>
        {#each SAVE_LOCATIONS as [platform, path]}
          <p>
            <b>{platform}:</b>
            <code>{@html path.replaceAll("/", "/<wbr />")}</code>
          </p>
        {/each}
      </div>
    {/if}

    <ShiplogCategories />

    <div class="block-wrapper">
      <label>
        <input
          type="checkbox"
          name="show-spoilers"
          class="other"
          bind:checked={$SETTINGS.hide_spoilers} />
        Hide spoilers
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
  .question-button {
    font-weight: 700;
  }
  .block-wrapper {
    background-color: var(--bg);
    border-radius: 10px;
    padding: 1px 10px;
    margin-bottom: 10px;
    max-width: 25em;

    & > h3 {
      margin-top: 10px;
    }
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
  code {
    word-break: break-word;
  }
  .hidden {
    display: none;
  }
</style>
