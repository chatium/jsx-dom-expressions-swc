import { createComponent as _$createComponent } from "r-server";
const template12 = _$createComponent(Comp, {
  get render() {
    return state.dynamic ? good() : bad;
  }
});
