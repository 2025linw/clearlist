import { Start } from "../generated/Start";

type StartType = Start["type"]
function toStart(type: StartType, date: string): Start {
  return  {
    'type': "On",
    'value': "2026-06-09",
  }
}
