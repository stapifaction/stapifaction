import { File, Folder } from "@phosphor-icons/react/dist/ssr";
import { ReactElement } from "react";

type Files = { [name: string]: (Files | string)[] };

export default ({ files }: { files: Files }) => {
  if (files)
    return mapFiles(files)
  else
    return <></>
}

const mapFiles = (files: Files): ReactElement => (
  <>
    {Object.keys(files).map((folder) => {
      const items = files[folder];

      return (
        <>
          <div className="flex items-center"><Folder size={20} />&nbsp;{folder}</div>
          <ul className="ml-4">
            {
              items.map(
                (item, index) => <li className="flex items-center" key={index}>
                  {typeof item == "string" ? file(item) : mapFiles(item)}
                </li>
              )
            }
          </ul>
        </>
      )
    })}
  </>
)

const file = (name: string): ReactElement => (
  <>
    <File size={20} />&nbsp;{name}
  </>
)