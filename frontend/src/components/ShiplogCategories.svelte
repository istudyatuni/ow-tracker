<script module>
  import { CATEGORIES, CATEGORY } from "@/lib/categories";
  import { t } from "@/lib/i18n";
  import {
    SAVE_FOUND,
    SAVE_FOUND_CATEGORIES,
    SAVE_KNOWN_CATEGORIES_NAMES,
    SELECTED_CATEGORIES,
    SETTINGS,
  } from "@/lib/stores";
</script>

<h4>{$t("shiplog-categories-header")}</h4>
{#each CATEGORIES as id (id)}
  <div
    class="spoiler"
    class:hidden={!$SAVE_FOUND_CATEGORIES.has(id) ||
      (id === CATEGORY.STRANGER && !$SAVE_FOUND && $SETTINGS.hide_dlc)}
    class:hide-spoilers={!$SAVE_KNOWN_CATEGORIES_NAMES.has(id)}>
    <label>
      <input
        type="checkbox"
        class={id}
        checked={$SELECTED_CATEGORIES[id]}
        onchange={(e) => SELECTED_CATEGORIES.set(id, e.target.checked)} />
      {$t(`shiplog-category-${id}`)}
    </label>
  </div>
{/each}

<style lang="scss">
  h4 {
    margin: 5px auto;
    margin-top: 0;
  }
</style>
