import Entry from "./Entry";
import Entries from "./Entries";

export default function Apple() {
  return (
    <Entries title="macOS / iOS">
      <Entry version="1.0.5" date={new Date("2024-06-13")}>
        This release introduces new Resource status updates in the Resource
        list.
      </Entry>
      <Entry version="1.0.4" date={new Date("2024-05-01")}>
        Bug fixes.
      </Entry>
      <Entry version="1.0.3" date={new Date("2024-04-28")}>
        Bug fixes.
      </Entry>
      <Entry version="1.0.2" date={new Date("2024-04-24")}>
        Bug fixes.
      </Entry>
      <Entry version="1.0.1" date={new Date("2024-04-04")}>
        Bug fixes.
      </Entry>
      <Entry version="1.0.0" date={new Date("2024-04-01")}>
        Initial release.
      </Entry>
    </Entries>
  );
}
