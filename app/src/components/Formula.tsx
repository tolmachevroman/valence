// A molecular formula with subscript counts and a superscript charge:
// C8H10N4O2, C2H3O2- (acetate).

const CHARGE = /([+-]\d*|\d*[+-])$/

export default function Formula({ formula }: { formula: string }) {
  const charge = formula.match(CHARGE)?.[0] ?? ''
  const body = formula.slice(0, formula.length - charge.length)
  return (
    <span className="formula">
      {body.split(/(\d+)/).map((part, i) =>
        /^\d+$/.test(part) ? <sub key={i}>{part}</sub> : part,
      )}
      {charge && <sup>{charge}</sup>}
    </span>
  )
}
