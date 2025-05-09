<script module>
  import { SETTINGS } from "@/lib/stores";
  import { t } from "@/lib/i18n";
</script>

<label>
  <input
    type="checkbox"
    name="show-spoilers"
    class="gray"
    bind:checked={$SETTINGS.hide_spoilers} />
  {$t("hide-spoilers-checkbox")}
</label>
