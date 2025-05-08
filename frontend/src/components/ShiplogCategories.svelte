<script module>
  import { CATEGORIES } from "../lib/categories";
  import { t } from "../lib/i18n";
  import { SAVE_FOUND_CATEGORIES, SELECTED_CATEGORIES } from "../lib/stores";
</script>

<script>
  let changed = $state(false);
</script>

<h4>{$t("shiplog-categories-header")}</h4>
{#each CATEGORIES as id (id)}
  <div class="spoiler" class:hidden={!$SAVE_FOUND_CATEGORIES.has(id)}>
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
<button onclick={() => window.location.reload()} disabled={!changed}
  >{$t("shiplog-categories-apply-button")}</button>

<style lang="scss">
  h4 {
    margin: 5px auto;
    margin-top: 0;
  }
  button {
    margin-top: 5px;
  }
</style>
