<script module>
  import SidebarApply from "./atoms/SidebarApply.svelte";

  import { CATEGORIES } from "../lib/categories";
  import { t } from "../lib/i18n";
  import {
    SAVE_FOUND_CATEGORIES,
    SAVE_KNOWN_CATEGORIES_NAMES,
    SELECTED_CATEGORIES,
  } from "../lib/stores";
</script>

<script>
  let changed = $state(false);
</script>

<h4>{$t("shiplog-categories-header")}</h4>
{#each CATEGORIES as id (id)}
  <div
    class="spoiler"
    class:hidden={!$SAVE_FOUND_CATEGORIES.has(id)}
    class:hide-spoilers={!$SAVE_KNOWN_CATEGORIES_NAMES.has(id)}>
    <label>
      <input
        type="checkbox"
        name={id}
        class={id}
        checked={$SELECTED_CATEGORIES[id]}
        onchange={(e) => {
          changed = true;
          SELECTED_CATEGORIES.set(id, e.target.checked);
        }} />
      {$t(`shiplog-category-${id}`)}
    </label>
  </div>
{/each}

<SidebarApply style="margin-top: 5px" disabled={!changed} />

<style lang="scss">
  h4 {
    margin: 5px auto;
    margin-top: 0;
  }
</style>
