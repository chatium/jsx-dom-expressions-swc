import { createComponent as _$createComponent } from "solid-js/web";
const template6 = _$createComponent(For, {
  get each() {
    return state.list;
  },
  get fallback() {
    return _$createComponent(Loading, {});
  },
  children: item => _$createComponent(Show, {
    get when() {
      return state.condition;
    },
    children: item
  })
});
