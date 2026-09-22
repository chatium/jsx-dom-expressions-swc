// The same const, declared before its use, does fold into the template.
const EARLIER_STYLES = `.a { color: red }`
const inOrder = <style>{EARLIER_STYLES}</style>
