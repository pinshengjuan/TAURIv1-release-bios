pub mod outlook {
    use anyhow::{
        anyhow, Context
    };
    use windows::{
        core::{
            HSTRING, BSTR, PCWSTR, GUID, VARIANT
        },
        Win32::System::{
            Com::{
                DISPATCH_PROPERTYPUT, DISPATCH_METHOD, CLSCTX_LOCAL_SERVER, DISPATCH_PROPERTYGET,
                IDispatch, DISPATCH_FLAGS, DISPPARAMS,
                CoInitialize, CLSIDFromProgID, CoCreateInstance
            },
            Ole::{
                DISPID_PROPERTYPUT
            }
        },
    };

    const LOCALE_USER_DEFAULT: u32 = 0x0400;
    const LOCALE_SYSTEM_DEFAULT: u32 = 0x0800;

    pub struct Variant(VARIANT);

    impl From<bool> for Variant {
        fn from(value: bool) -> Self {
            Self(value.into())
        }
    }

    impl From<i32> for Variant {
        fn from(value: i32) -> Self {
            Self(value.into())
        }
    }

    impl From<&str> for Variant {
        fn from(value: &str) -> Self {
            Self(BSTR::from(value).into())
        }
    }

    impl From<&String> for Variant {
        fn from(value: &String) -> Self {
            Self(BSTR::from(value).into())
        }
    }

    impl Variant {
        pub fn int(&self) -> anyhow::Result<i32> {
            Ok(i32::try_from(&self.0)?)
        }

        pub fn string(&self) -> anyhow::Result<String> {
            Ok(BSTR::try_from(&self.0)?.to_string())
        }

        pub fn idispatch(&self) -> anyhow::Result<IDispatchWrapper> {
            Ok(IDispatchWrapper(IDispatch::try_from(&self.0)?))
        }
    }

    pub struct IDispatchWrapper(pub IDispatch);

    impl IDispatchWrapper {
        //
        // refer: https://learn.microsoft.com/en-us/dotnet/api/system.windows.threading.dispatcher.invoke?view=windowsdesktop-8.0
        //
        pub fn invoke(&self, flags: DISPATCH_FLAGS, name: &str, mut args: Vec<Variant>) -> anyhow::Result<Variant> {
            unsafe {
                let mut dispid = 0;
                self.0
                    .GetIDsOfNames(&GUID::default(),
                                    &PCWSTR::from_raw(HSTRING::from(name).as_ptr()),
                                    1,
                                    LOCALE_USER_DEFAULT,
                                    &mut dispid,)
                    .with_context(|| "GetIDsOfNames")?;

                let mut dp = DISPPARAMS::default();
                let mut dispid_named = DISPID_PROPERTYPUT;

                if !args.is_empty() {
                    args.reverse();
                    dp.cArgs = args.len() as u32;
                    dp.rgvarg = args.as_mut_ptr() as *mut VARIANT;

                    // Handle special-case for property "put"
                    if (flags & DISPATCH_PROPERTYPUT) != DISPATCH_FLAGS(0) {
                        dp.cNamedArgs = 1;
                        dp.rgdispidNamedArgs = &mut dispid_named;
                    }
                }

                let mut result = VARIANT::default();
                self.0
                    .Invoke(dispid,
                            &GUID::default(),
                            LOCALE_SYSTEM_DEFAULT,
                            flags,
                            &dp,
                            Some(&mut result),
                            None,
                            None,)
                    .with_context(|| "Invoke")?;

                Ok(Variant(result))
            }
        }

        pub fn get(&self, name: &str) -> anyhow::Result<Variant> {
            self.invoke(DISPATCH_PROPERTYGET, name, vec![])
        }

        pub fn put(&self, name: &str, args: Vec<Variant>) -> anyhow::Result<Variant> {
            self.invoke(DISPATCH_PROPERTYPUT, name, args)
        }

        pub fn call(&self, name: &str, args: Vec<Variant>) -> anyhow::Result<Variant> {
            self.invoke(DISPATCH_METHOD, name, args)
        }
    }

    pub fn mail(to: String, cc: String, subject: String, body: String) -> anyhow::Result<()> {
        //
        // Refer: 
        // https://learn.microsoft.com/en-us/office/vba/api/outlook.mailitem
        // https://learn.microsoft.com/en-us/office/vba/api/outlook.mailitem.to
        // https://learn.microsoft.com/en-us/office/vba/api/outlook.mailitem.cc
        // https://learn.microsoft.com/en-us/office/vba/api/outlook.mailitem.subject
        // https://learn.microsoft.com/en-us/office/vba/api/outlook.mailitem.htmlbody
        // https://learn.microsoft.com/en-us/office/vba/api/outlook.mailitem.display
        //
        unsafe {
            let res = CoInitialize(None);
            if res.is_err() {
                return Err(anyhow!("error: {}", res.message()));
            }
    
            let clsid = CLSIDFromProgID(PCWSTR::from_raw(
                HSTRING::from("Outlook.Application").as_ptr(),
            ))
            .with_context(|| "CLSIDFromProgID")?;

            // println!("{:?}", clsid);

            let outlook = CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER)
                .with_context(|| "CoCreateInstance")?;
            let outlook = IDispatchWrapper(outlook);

            let new_mail_item: i32 = 0;
            let result = outlook.call("CreateItem", vec![new_mail_item.into()]).with_context(|| "call CreateItem")?;
            let create_item = result.idispatch().with_context(|| "idispatch CreateItem")?;

            let to = to.as_str();
            create_item.put("To", vec![to.into()]).with_context(|| "put recipients")?;

            let cc = cc.as_str();
            create_item.put("CC", vec![cc.into()]).with_context(|| "put recipients")?;

            let subject = subject.as_str();
            create_item.put("Subject", vec![(subject).into()]).with_context(|| "put subject")?;

            let mailbody = body.as_str();
            create_item.put("HTMLBody", vec![(mailbody).into()]).with_context(|| "put html body")?;

            create_item.call("Display", vec![0.into()]).with_context(|| "call Dsiplay")?;
            Ok(())
        }
    }
}