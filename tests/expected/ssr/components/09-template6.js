import { Show as _$Show } from "r-server";
import { createComponent as _$createComponent } from "r-server";
import { For as _$For } from "r-server";
const template6 = _$createComponent(_$For, {
  get each() {
    return state.list;
  },
  get fallback() {
    return _$createComponent(Loading, {});
  },
  children: item => _$createComponent(_$Show, {
    get when() {
      return state.condition;
    },
    children: item
  })
});
