use std::fmt::{Display, Formatter};

pub enum Usage {
    /// For own purposes
    Custom(String),
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,
    /// View your data across Google Cloud services and see the email address of your Google Account
    CloudPlatformReadOnly,
    /// Manage your Ad Exchange buyer account configuration
    AdExchangeBuyer,
    /// See your AdMob data
    AdMobReadOnly,
    /// See your AdMob data
    AdMobReport,
    /// View and manage your AdSense host data and associated accounts
    AdSenseHost,
    /// View audit reports for your G Suite domain
    AdminReportsAuditReadOnly,
    /// View audit reports for your G Suite domain
    AdminReportsUsageReadOnly,
    /// View and manage data transfers between users in your organization
    AdminDataTransfer,
    /// View data transfers between users in your organization
    AdminDataTransferReadOnly,
    /// See, add, edit, and permanently delete the printers that your organization can use with Chrome
    AdminChromePrinters,
    /// See the printers that your organization can use with Chrome
    AdminChromePrintersReadOnly,
    /// View and manage customer related information
    AdminDirectoryCustomer,
    /// View customer related information
    AdminDirectoryCustomerReadOnly,
    /// View and manage your Chrome OS devices' metadata
    AdminDirectoryDeviceChromeOS,
    /// View your Chrome OS devices' metadata
    AdminDirectoryDeviceChromeOSReadOnly,
    /// View and manage your mobile devices' metadata
    AdminDirectoryDeviceMobile,
    /// Manage your mobile devices by performing administrative tasks
    AdminDirectoryDeviceMobileAction,
    /// View your mobile devices' metadata
    AdminDirectoryDeviceMobileReadOnly,
    /// View and manage the provisioning of domains for your customers
    AdminDirectoryDomain,
    /// View domains related to your customers
    AdminDirectoryDomainReadOnly,
    /// View and manage the provisioning of groups on your domain
    AdminDirectoryGroup,
    /// View and manage group subscriptions on your domain
    AdminDirectoryGroupMember,
    /// View group subscriptions on your domain
    AdminDirectoryGroupMemberReadOnly,
    /// View groups on your domain
    AdminDirectoryGroupReadOnly,
    /// View and manage organization units on your domain
    AdminDirectoryOrgUnit,
    /// View organization units on your domain
    AdminDirectoryOrgUnitReadOnly,
    /// View and manage the provisioning of calendar resources on your domain
    AdminDirectoryResourceCalendar,
    /// View calendar resources on your domain
    AdminDirectoryResourceCalendarReadOnly,
    /// Manage delegated admin roles for your domain
    AdminDirectoryRoleManagement,
    /// View delegated admin roles for your domain
    AdminDirectoryRoleManagementReadOnly,
    /// View and manage the provisioning of users on your domain
    AdminDirectoryUser,
    /// View and manage user aliases on your domain
    AdminDirectoryUserAlias,
    /// View user aliases on your domain
    AdminDirectoryUserAliasReadOnly,
    /// See info about users on your domain
    AdminDirectoryUserReadOnly,
    /// Manage data access permissions for users on your domain
    AdminDirectoryUserSecurity,
    /// View and manage the provisioning of user schemas on your domain
    AdminDirectoryUserSchema,
    /// View user schemas on your domain
    AdminDirectoryUserSchemaReadOnly,
    /// View and manage your Google Analytics data
    Analytics,
    /// See and download your Google Analytics data
    AnalyticsReadOnly,
    /// Manage Android devices and apps for your customers
    AndroidManagement,
    /// View and manage your applications deployed on Google App Engine
    AppEngineAdmin,
    /// Read, compose, send, and permanently delete all your email from Gmail
    Mail,
    /// See, edit, share, and permanently delete all the calendars you can access using Google Calendar
    CalendarFeeds,
    /// See, edit, download, and permanently delete your contacts
    M8Feeds,
    /// See, edit, create, and delete all your Google Docs documents
    Documents,
    /// See, edit, create, and delete all of your Google Drive files
    Drive,
    /// View and manage your forms in Google Drive
    Forms,
    /// View and manage forms that this application has been installed in
    FormsCurrentOnly,
    /// View and manage your Google Groups
    Groups,
    /// Create and update Google Apps Script deployments
    ScriptDeployments,
    /// View Google Apps Script deployments
    ScriptDeploymentsReadOnly,
    /// View Google Apps Script project's metrics
    ScriptMetrics,
    /// View Google Apps Script processes
    ScriptProcesses,
    /// Create and update Google Apps Script projects
    ScriptProjects,
    /// View Google Apps Script projects
    ScriptProjectsReadOnly,
    /// See, edit, create, and delete all your Google Sheets spreadsheets
    Spreadsheets,
    /// See your primary Google Account email address
    UserInfoEmail,
    /// View and manage your data in Google BigQuery and see the email address for your Google Account
    BigQuery,
    /// Insert data into Google BigQuery
    BigQueryInsertData,
    /// Manage your data and permissions in Cloud Storage and see the email address for your Google Account
    DevStorageFullControl,
    /// View your data in Google Cloud Storage
    DevStorageReadOnly,
    /// Manage your data in Cloud Storage and see the email address of your Google Account
    DevStorageReadWrite,
    /// Manage your Blogger account
    Blogger,
    /// View your Blogger account
    BloggerReadOnly,
    /// Manage your books
    Books,
    /// See, edit, share, and permanently delete all the calendars you can access using Google Calendar
    Calendar,
    /// View and edit events on all your calendars
    CalendarEvents,
    /// View events on all your calendars
    CalendarEventsReadOnly,
    /// See and download any calendar you can access using your Google Calendar
    CalendarReadOnly,
    /// View your Calendar settings
    CalendarSettingsReadOnly,
    /// Manage DoubleClick Digital Marketing conversions
    DdmConversions,
    /// View and manage DoubleClick for Advertisers reports
    DfaReporting,
    /// View and manage your DoubleClick Campaign Manager's (DCM) display ad campaigns
    DfaTrafficking,
    /// Administer your Cloud Bigtable tables and clusters
    BigTableAdmin,
    /// Administer your Cloud Bigtable clusters
    BigTableAdminCluster,
    /// Administer your Cloud Bigtable clusters
    BigTableAdminInstance,
    /// Administer your Cloud Bigtable tables
    BigTableAdminTable,
    /// Administer your Cloud Bigtable tables and clusters
    CloudBigTableAdmin,
    /// Administer your Cloud Bigtable clusters
    CloudBigTableAdminCluster,
    /// Administer your Cloud Bigtable tables
    CloudBigTableAdminTable,
    /// View and manage your Google Cloud Platform billing accounts
    CloudBilling,
    /// View your Google Cloud Platform billing accounts
    CloudBillingReadOnly,
    /// View your DNS records hosted by Google Cloud DNS
    NDevCloudDnsReadOnly,
    /// View and manage your DNS records hosted by Google Cloud DNS
    NDevCloudDnsReadWrite,
    /// Use Stack driver Debugger
    CloudDebugger,
    /// View and manage your Google Cloud Platform management resources and deployment status information
    NDevCloudMan,
    /// View your Google Cloud Platform management resources and deployment status information
    NDevCloudManReadOnly,
    /// View and manage your Google Cloud Datastore data
    DataStore,
    /// See your device details
    CloudIdentityDevicesLookup,
    /// See, change, create, and delete any of the Cloud Identity Groups that you can access, including the members of each group
    CloudIdentityGroups,
    /// See any Cloud Identity Groups that you can access, including group members and their emails
    CloudIdentityGroupsReadOnly,
    /// View and manage your keys and secrets stored in Cloud Key Management Service
    CloudKms,
    /// Submit log data for your projects
    LoggingWrite,
    /// View log data for your projects
    LoggingRead,
    /// Administrate log data for your projects
    LoggingAdmin,
    /// View and write monitoring data for all of your Google and third-party Cloud and API projects
    Monitoring,
    /// View monitoring data for all of your Google Cloud and third-party projects
    MonitoringRead,
    /// Publish metric data to your Google Cloud projects
    MonitoringWrite,
    /// Apply machine learning models to reveal the structure and meaning of text
    CloudLanguage,
    /// View and manage your Google Compute Engine resources
    Compute,
    /// View your Google Compute Engine resources
    ComputeReadOnly,
    /// View and manage Pub/Sub topics and subscriptions
    PubSub,
    /// Manage your Google Cloud Platform services' runtime configuration
    CloudRuntimeConfig,
    /// Manage your Google SQL Service instances
    SqlServiceAdmin,
    /// Index and serve your organization's data with Cloud Search
    CloudSearch,
    /// Index and serve your organization's data with Cloud Search
    CloudSearchDebug,
    /// Index and serve your organization's data with Cloud Search
    CloudSearchIndexing,
    /// Search your organization's data in the Cloud Search index
    CloudSearchQuery,
    /// Index and serve your organization's data with Cloud Search
    CloudSearchSettings,
    /// Index and serve your organization's data with Cloud Search
    CloudSearchSettingsIndexing,
    /// Index and serve your organization's data with Cloud Search
    CloudSearchSettingsQuery,
    /// Index and serve your organization's data with Cloud Search
    CloudSearchStats,
    /// Index and serve your organization's data with Cloud Search
    CloudSearchStatsIndexing,
    /// Manage your source code repositories
    SourceFullControl,
    /// View the contents of your source code repositories
    SourceReadOnly,
    /// Manage the contents of your source code repositories
    SourceReadWrite,
    /// Administer your Spanner databases
    SpannerAdmin,
    /// View and manage the contents of your Spanner databases
    SpannerData,
    /// Write Trace data for a project or application
    TraceAppend,
    /// Translate text from one language to another using Google Translate
    CloudTranslation,
    /// Apply machine learning models to understand and label images
    CloudVision,
    /// Manage your product listings and accounts for Google Shopping
    Content,
    /// View and add to the activity record of files in your Google Drive
    DriveActivity,
    /// View the activity record of files in your Google Drive
    DriveActivityReadOnly,
    /// View and manage G Suite licenses for your domain
    AppsLicensing,
    /// Send messages and manage messaging subscriptions for your Firebase applications
    FirebaseMessaging,
    /// View and administer all your Firebase data and settings
    Firebase,
    /// View all your Firebase data and settings
    FirebaseReadOnly,
    /// Use Google Fit to see and store your physical activity data
    FitnessActivityRead,
    /// Add to your Google Fit physical activity data
    FitnessActivityWrite,
    /// See info about your blood glucose in Google Fit. I consent to Google sharing my blood glucose information with this app.
    FitnessBloodGlucoseRead,
    /// Add info about your blood glucose to Google Fit. I consent to Google using my blood glucose information with this app.
    FitnessBloodGlucoseWrite,
    /// See info about your blood pressure in Google Fit. I consent to Google sharing my blood pressure information with this app.
    FitnessBloodPressureRead,
    /// Add info about your blood pressure in Google Fit. I consent to Google using my blood pressure information with this app.
    FitnessBloodPressureWrite,
    /// See info about your body measurements in Google Fit
    FitnessBodyRead,
    /// Add info about your body measurements to Google Fit
    FitnessBodyWrite,
    /// See info about your body temperature in Google Fit. I consent to Google sharing my body temperature information with this app.
    FitnessBodyTemperatureRead,
    /// Add to info about your body temperature in Google Fit. I consent to Google using my body temperature information with this app.
    FitnessBodyTemperatureWrite,
    /// See your heart rate data in Google Fit. I consent to Google sharing my heart rate information with this app.
    FitnessHeartRateRead,
    /// Add to your heart rate data in Google Fit. I consent to Google using my heart rate information with this app.
    FitnessHeartRateWrite,
    /// Add to your Google Fit location data
    FitnessLocationWrite,
    /// See your Google Fit speed and distance data
    FitnessLocationRead,
    /// See info about your nutrition in Google Fit
    FitnessNutritionRead,
    /// Add to info about your nutrition in Google Fit
    FitnessNutritionWrite,
    /// See info about your oxygen saturation in Google Fit. I consent to Google sharing my oxygen saturation information with this app.
    FitnessOxygenSaturationRead,
    /// Add info about your oxygen saturation in Google Fit. I consent to Google using my oxygen saturation information with this app.
    FitnessOxygenSaturationWrite,
    /// See info about your reproductive health in Google Fit. I consent to Google sharing my reproductive health information with this app.
    FitnessReproductiveHealthRead,
    /// Add info about your reproductive health in Google Fit. I consent to Google using my reproductive health information with this app.
    FitnessReproductiveHealthWrite,
    /// See your sleep data in Google Fit. I consent to Google sharing my sleep information with this app.
    FitnessSleepRead,
    /// Add to your sleep data in Google Fit. I consent to Google using my sleep information with this app.
    FitnessSleepWrite,
    /// View and manage Genomics data
    Genomics,
    /// Manage drafts and send emails when you interact with the add-on
    MailAddonsCurrentActionCompose,
    /// View your email messages when you interact with the add-on
    MailAddonsCurrentMessageAction,
    /// View your email message metadata when the add-on is running
    MailAddonsCurrentMessageMetaData,
    /// View your email messages when the add-on is running
    MailAddonsCurrentMessageReadOnly,
    /// Manage drafts and send emails
    MailCompose,
    /// Add emails into your Gmail mailbox
    MailInsert,
    /// See and edit your email labels
    MailLabels,
    /// View your email message metadata such as labels and headers, but not the email body
    MailMetaData,
    /// Read, compose, and send emails from your Gmail account
    MailModify,
    /// iew your email messages and settings
    MailReadOnly,
    /// Send email on your behalf
    MailSend,
    /// See, edit, create, or change your email settings and filters in Gmail
    MailSettingsBasic,
    /// Manage your sensitive mail settings, including who can manage your mail
    MailSettingsSharing,
    /// Edit Google Analytics management entities
    AnalyticsEdit,
    /// Manage Google Analytics Account users by email address
    AnalyticsManageUsers,
    /// View Google Analytics user permissions
    AnalyticsManageUsersReadOnly,
    /// Create a new Google Analytics account along with its default property and view
    AnalyticsProVision,
    /// Manage Google Analytics user deletion requests
    AnalyticsUserDeletion,
    /// Delete conversations and spaces & remove access to associated files in Google Chat
    ChatDelete,
    /// View members in Google Chat conversations.
    ChatMembershipsReadOnly,
    /// View, add, and remove members from conversations in Google Chat
    ChatMemberships,
    /// Add and remove itself from conversations in Google Chat
    ChatMembershipsApp,
    /// View, compose, send, update, and delete messages, and add, view, and delete reactions to messages.
    ChatMessages,
    /// Compose and send messages in Google Chat
    ChatMessagesCreate,
    /// View, add, and delete reactions to messages in Google Chat
    ChatMessagesReactions,
    /// Add reactions to messages in Google Chat
    ChatMessagesReactionsCreate,
    /// View reactions to messages in Google Chat
    ChatMessagesReactionsReadOnly,
    /// View messages and reactions in Google Chat
    ChatMessagesReadOnly,
    /// Create conversations and spaces and view or update metadata (including history settings) in Google Chat
    ChatSpaces,
    /// View and manage announcements in Google Classroom
    ClassroomAnnouncements,
    /// View chat and spaces in Google Chat
    ChatSpacesReadOnly,
    /// Create new conversations in Google Chat
    ChatSpacesCreate,
    /// View announcements in Google Classroom
    ClassroomAnnouncementsReadOnly,
    /// See, edit, create, and permanently delete your Google Classroom classes
    ClassroomCourses,
    /// View your Google Classroom classes
    ClassroomCoursesReadOnly,
    /// See, create and edit coursework items including assignments, questions, and grades
    ClassroomCourseworkMe,
    /// View your course work and grades in Google Classroom
    ClassroomCourseworkMeReadOnly,
    /// Manage course work and grades for students in the Google Classroom classes you teach and view the course work and grades for classes you administer
    ClassroomCourseworkStudents,
    /// Google Classroom classes you teach or administer
    ClassroomCourseworkStudentsReadOnly,
    /// See, edit, and create classwork materials in Google Classroom
    ClassroomCourseworkMaterials,
    /// See all classwork materials for your Google Classroom classes
    ClassroomCourseworkMaterialsReadOnly,
    /// View your Google Classroom guardians
    ClassroomGuardianLinksMeReadOnly,
    /// View and manage guardians for students in your Google Classroom classes
    ClassroomGuardianLinksStudents,
    /// View guardians for students in your Google Classroom classes
    ClassroomGuardianLinksStudentsReadOnly,
    /// View the email addresses of people in your classes
    ClassroomProfileEmails,
    /// View the profile photos of people in your classes
    ClassroomProfilePhotos,
    /// Receive notifications about your Google Classroom data
    ClassroomPushNotifications,
    /// Manage your Google Classroom class rosters
    ClassroomRosters,
    /// View your Google Classroom class rosters
    ClassroomRostersReadOnly,
    /// View your course work and grades in Google Classroom
    ClassroomStudentSubmissionsMeReadOnly,
    /// View course work and grades for students in the Google Classroom classes you teach or administer
    ClassroomStudentSubmissionsStudentsReadOnly,
    /// See, create, and edit topics in Google Classroom
    ClassroomTopics,
    /// View topics in Google Classroom
    ClassroomTopicsReadOnly,
    /// View YouTube Analytics reports for your YouTube content
    YoutubeAnalyticsReadOnly,
    /// View monetary and non-monetary YouTube Analytics reports for your YouTube content
    YoutubeAnalyticsMonetaryReadOnly,
    /// View private information of your YouTube channel relevant during the audit process with a YouTube partner
    YoutubePartnerChannelAudit,
    /// See, edit, and permanently delete your YouTube videos, ratings, comments and captions
    YouTubeForceSsl,
    /// View your YouTube account
    YoutubeReadOnly,
    /// Manage your YouTube videos
    YoutubeUpload,
    /// View and manage your assets and associated content on YouTube
    YoutubePartner,
    /// See a list of your current active channel members, their current level, and when they became a member
    YoutubeChannelMembershipsCreator,
    /// Manage your YouTube account
    Youtube,
    /// Publish your Google Tag Manager container versions
    TagManagerPublish,
    /// View your Google Tag Manager container and its subcomponents
    TagManagerReadOnly,
    /// Manage user permissions of your Google Tag Manager account and container
    TagManagerManageUsers,
    /// View and manage your Google Tag Manager accounts
    TagManagerManageAccounts,
    /// Manage your Google Tag Manager container versions
    TagManagerEditContainerVersions,
    /// Manage your Google Tag Manager container and its subcomponents, excluding versioning and publishing
    TagManagerEditContainers,
    /// Delete your Google Tag Manager containers
    TagManagerDeleteContainers,
    /// Publish and manage your 360 photos on Google Street View
    StreetViewPublish,
    /// Manage your Google API service configuration
    ServiceManagement,
    /// View your Google API service configuration
    ServiceManagementReadOnly,
    /// View and manage your advertising data in DoubleClick Search
    DoubleClickSearch,
    /// Read, create, update, and delete your SAS Portal data.
    SasPortal,
    /// Manage and add to shared albums on your behalf
    PhotosLibrarySharing,
    /// Manage photos added by this app
    PhotosLibraryReadOnlyAppCreatedData,
    /// View your Google Photos library
    PhotosLibraryReadOnly,
    /// Edit the info in your photos, videos, and albums created within this app, including titles, descriptions, and covers
    PhotosLibraryEditAppCreatedData,
    /// Add to your Google Photos library
    PhotosLibraryAppendOnly,
    /// See, upload, and organize items in your Google Photos library
    PhotosLibrary,
    /// See your personal info, including any personal info you've made publicly available
    UserInfoProfile,
    /// See and download your personal phone numbers
    UserPhoneNumbersRead,
    /// See your education, work history and org info
    UserOrganizationRead,
    /// See and download your contacts
    ContactsReadOnly,
    /// See and download your organization's GSuite directory
    DirectoryReadOnly,
    /// View your street addresses
    UserAddressesRead,
    /// See and download your exact date of birth
    UserBirthdayRead,
    /// See and download all of your Google Account email addresses
    UserEmailsRead,
    /// See your gender
    UserGenderRead,
    /// Manage your product listings for Google Manufacturer Center
    ManufacturerCenter,
    /// See and download contact info automatically saved in your "Other contacts"
    ContactsOtherReadOnly,
    /// Submit data to Google for indexing
    Indexing,
    /// See and delete your domain's G Suite alerts, and send alert feedback
    AppsAlerts,
    /// Manage users on your domain
    AppsOrder,
    /// Manage users on your domain
    AppsOrderReadOnly,
    /// Upload messages to any Google group in your domain
    AppsGroupsMigration,
    /// View and manage the settings of a G Suite group
    AppsGroupsSettings,
    /// View your eDiscovery data
    EDiscoveryReadOnly,
    /// Manage your eDiscovery data
    EDiscovery,
    /// Create, edit, organize, and delete all your tasks
    Tasks,
    /// View your tasks
    TasksReadOnly,
    /// See all your Google Sheets spreadsheets
    SpreadsheetsReadOnly,
    /// See, edit, create, and delete all your Google Slides presentations
    Presentations,
    /// See all your Google Slides presentations
    PresentationsReadOnly,
    /// See and download all your Google Drive files
    DriveReadOnly,
    /// See, edit, create, and delete only the specific Google Drive files you use with this app
    DriveFile,
    /// Manage your new site verifications with Google
    SiteVerificationVerifyOnly,
    /// Manage the list of sites and domains you control
    SiteVerification,
    /// View Search Console data for your verified sites
    WebmastersReadOnly,
    /// View and manage Search Console data for your verified sites
    Webmasters,
    /// View and manage your Google Play Developer account
    AndroidPublisher,
    /// See, create, and delete its own configuration data in your Google Drive
    DriveAppData,
    /// Create, edit, and delete your Google Play Games activity
    Games,
    /// Manage corporate Android devices
    AndroidEnterprise,
    /// Modify your Google Apps Script scripts' behavior
    DriveScripts,
    /// View the photos, videos and albums in your Google Photos
    DrivePhotosReadOnly,
    /// See information about your Google Drive files
    DriveMetaDataReadOnly,
    /// View and manage metadata of files in your Google Drive
    DriveMetaData,
    /// See all your Google Docs documents
    DocumentsReadOnly,
    /// See your primary Google Account email address
    Email,
    /// Associate you with your personal info on Google
    OpenId,
    /// See your personal info, including any personal info you've made publicly available
    Profile,
}

impl Display for Usage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{url}", url = self.as_string())
    }
}


impl Usage {
    pub(crate) fn as_string(&self) -> String {
        match self {
            Usage::Custom(url) => url.clone(),
            Usage::CloudPlatform => String::from("https://www.googleapis.com/auth/cloud-platform"),
            Usage::CloudPlatformReadOnly => {
                String::from("https://www.googleapis.com/auth/cloud-platform.read-only")
            }
            Usage::AdExchangeBuyer => {
                String::from("https://www.googleapis.com/auth/adexchange.buyer")
            }
            Usage::AdMobReadOnly => String::from("https://www.googleapis.com/auth/admob.readonly"),
            Usage::AdMobReport => String::from("https://www.googleapis.com/auth/admob.report"),
            Usage::AdSenseHost => String::from("https://www.googleapis.com/auth/adsensehost"),
            Usage::AdminReportsAuditReadOnly => {
                String::from("https://www.googleapis.com/auth/admin.reports.audit.readonly")
            }
            Usage::AdminReportsUsageReadOnly => {
                String::from("https://www.googleapis.com/auth/admin.reports.usage.readonly")
            }
            Usage::AdminDataTransfer => {
                String::from("https://www.googleapis.com/auth/admin.datatransfer")
            }
            Usage::AdminDataTransferReadOnly => {
                String::from("https://www.googleapis.com/auth/admin.datatransfer.readonly")
            }
            Usage::AdminChromePrinters => {
                String::from("https://www.googleapis.com/auth/admin.chrome.printers")
            }
            Usage::AdminChromePrintersReadOnly => {
                String::from("https://www.googleapis.com/auth/admin.chrome.printers.readonly")
            }
            Usage::AdminDirectoryCustomer => {
                String::from("https://www.googleapis.com/auth/admin.directory.customer")
            }
            Usage::AdminDirectoryCustomerReadOnly => {
                String::from("https://www.googleapis.com/auth/admin.directory.customer.readonly")
            }
            Usage::AdminDirectoryDeviceChromeOS => {
                String::from("https://www.googleapis.com/auth/admin.directory.device.chromeos")
            }
            Usage::AdminDirectoryDeviceChromeOSReadOnly => String::from(
                "https://www.googleapis.com/auth/admin.directory.device.chromeos.readonly",
            ),
            Usage::AdminDirectoryDeviceMobile => {
                String::from("https://www.googleapis.com/auth/admin.directory.device.mobile")
            }
            Usage::AdminDirectoryDeviceMobileAction => {
                String::from("https://www.googleapis.com/auth/admin.directory.device.mobile.action")
            }
            Usage::AdminDirectoryDeviceMobileReadOnly => String::from(
                "https://www.googleapis.com/auth/admin.directory.device.mobile.readonly",
            ),
            Usage::AdminDirectoryDomain => {
                String::from("https://www.googleapis.com/auth/admin.directory.domain")
            }
            Usage::AdminDirectoryDomainReadOnly => {
                String::from("https://www.googleapis.com/auth/admin.directory.domain.readonly")
            }
            Usage::AdminDirectoryGroup => {
                String::from("https://www.googleapis.com/auth/admin.directory.group")
            }
            Usage::AdminDirectoryGroupMember => {
                String::from("https://www.googleapis.com/auth/admin.directory.group.member")
            }
            Usage::AdminDirectoryGroupMemberReadOnly => String::from(
                "https://www.googleapis.com/auth/admin.directory.group.member.readonly",
            ),
            Usage::AdminDirectoryGroupReadOnly => {
                String::from("https://www.googleapis.com/auth/admin.directory.group.readonly")
            }
            Usage::AdminDirectoryOrgUnit => {
                String::from("https://www.googleapis.com/auth/admin.directory.orgunit")
            }
            Usage::AdminDirectoryOrgUnitReadOnly => {
                String::from("https://www.googleapis.com/auth/admin.directory.orgunit.readonly")
            }
            Usage::AdminDirectoryResourceCalendar => {
                String::from("https://www.googleapis.com/auth/admin.directory.resource.calendar")
            }
            Usage::AdminDirectoryResourceCalendarReadOnly => String::from(
                "https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly",
            ),
            Usage::AdminDirectoryRoleManagement => {
                String::from("https://www.googleapis.com/auth/admin.directory.rolemanagement")
            }
            Usage::AdminDirectoryRoleManagementReadOnly => String::from(
                "https://www.googleapis.com/auth/admin.directory.rolemanagement.readonly",
            ),
            Usage::AdminDirectoryUser => {
                String::from("https://www.googleapis.com/auth/admin.directory.user")
            }
            Usage::AdminDirectoryUserAlias => {
                String::from("https://www.googleapis.com/auth/admin.directory.user.alias")
            }
            Usage::AdminDirectoryUserAliasReadOnly => {
                String::from("https://www.googleapis.com/auth/admin.directory.user.alias.readonly")
            }
            Usage::AdminDirectoryUserReadOnly => {
                String::from("https://www.googleapis.com/auth/admin.directory.user.readonly")
            }
            Usage::AdminDirectoryUserSecurity => {
                String::from("https://www.googleapis.com/auth/admin.directory.user.security")
            }
            Usage::AdminDirectoryUserSchema => {
                String::from("https://www.googleapis.com/auth/admin.directory.userschema")
            }
            Usage::AdminDirectoryUserSchemaReadOnly => {
                String::from("https://www.googleapis.com/auth/admin.directory.userschema.readonly")
            }
            Usage::Analytics => String::from("https://www.googleapis.com/auth/analytics"),
            Usage::AnalyticsReadOnly => {
                String::from("https://www.googleapis.com/auth/analytics.readonly")
            }
            Usage::AndroidManagement => {
                String::from("https://www.googleapis.com/auth/androidmanagement")
            }
            Usage::AppEngineAdmin => {
                String::from("https://www.googleapis.com/auth/appengine.admin")
            }
            Usage::Mail => String::from("https://mail.google.com/"),
            Usage::CalendarFeeds => String::from("https://www.google.com/calendar/feeds"),
            Usage::M8Feeds => String::from("https://www.google.com/m8/feeds"),
            Usage::Documents => String::from("https://www.googleapis.com/auth/documents"),
            Usage::Drive => String::from("https://www.googleapis.com/auth/drive"),
            Usage::Forms => String::from("https://www.googleapis.com/auth/forms"),
            Usage::FormsCurrentOnly => {
                String::from("https://www.googleapis.com/auth/forms.currentonly")
            }
            Usage::Groups => String::from("https://www.googleapis.com/auth/groups"),
            Usage::ScriptDeployments => {
                String::from("https://www.googleapis.com/auth/script.deployments")
            }
            Usage::ScriptDeploymentsReadOnly => {
                String::from("https://www.googleapis.com/auth/script.deployments.readonly")
            }
            Usage::ScriptMetrics => String::from("https://www.googleapis.com/auth/script.metrics"),
            Usage::ScriptProcesses => {
                String::from("https://www.googleapis.com/auth/script.processes")
            }
            Usage::ScriptProjects => {
                String::from("https://www.googleapis.com/auth/script.projects")
            }
            Usage::ScriptProjectsReadOnly => {
                String::from("https://www.googleapis.com/auth/script.projects.readonly")
            }
            Usage::Spreadsheets => String::from("https://www.googleapis.com/auth/spreadsheets"),
            Usage::UserInfoEmail => String::from("https://www.googleapis.com/auth/userinfo.email"),
            Usage::BigQuery => String::from("https://www.googleapis.com/auth/bigquery"),
            Usage::BigQueryInsertData => {
                String::from("https://www.googleapis.com/auth/bigquery.insertdata")
            }
            Usage::DevStorageFullControl => {
                String::from("https://www.googleapis.com/auth/devstorage.full_control")
            }
            Usage::DevStorageReadOnly => {
                String::from("https://www.googleapis.com/auth/devstorage.read_only")
            }
            Usage::DevStorageReadWrite => {
                String::from("https://www.googleapis.com/auth/devstorage.read_write")
            }
            Usage::Blogger => String::from("https://www.googleapis.com/auth/blogger"),
            Usage::BloggerReadOnly => {
                String::from("https://www.googleapis.com/auth/blogger.readonly")
            }
            Usage::Books => String::from("https://www.googleapis.com/auth/books"),
            Usage::Calendar => String::from("https://www.googleapis.com/auth/calendar"),
            Usage::CalendarEvents => {
                String::from("https://www.googleapis.com/auth/calendar.events")
            }
            Usage::CalendarEventsReadOnly => {
                String::from("https://www.googleapis.com/auth/calendar.events.readonly")
            }
            Usage::CalendarReadOnly => {
                String::from("https://www.googleapis.com/auth/calendar.readonly")
            }
            Usage::CalendarSettingsReadOnly => {
                String::from("https://www.googleapis.com/auth/calendar.settings.readonly")
            }
            Usage::DdmConversions => String::from("https://www.googleapis.com/auth/ddmconversions"),
            Usage::DfaReporting => String::from("https://www.googleapis.com/auth/dfareporting"),
            Usage::DfaTrafficking => String::from("https://www.googleapis.com/auth/dfatrafficking"),
            Usage::BigTableAdmin => String::from("https://www.googleapis.com/auth/bigtable.admin"),
            Usage::BigTableAdminCluster => {
                String::from("https://www.googleapis.com/auth/bigtable.admin.cluster")
            }
            Usage::BigTableAdminInstance => {
                String::from("https://www.googleapis.com/auth/bigtable.admin.instance")
            }
            Usage::BigTableAdminTable => {
                String::from("https://www.googleapis.com/auth/bigtable.admin.table")
            }
            Usage::CloudBigTableAdmin => {
                String::from("https://www.googleapis.com/auth/cloud-bigtable.admin")
            }
            Usage::CloudBigTableAdminCluster => {
                String::from("https://www.googleapis.com/auth/cloud-bigtable.admin.cluster")
            }
            Usage::CloudBigTableAdminTable => {
                String::from("https://www.googleapis.com/auth/cloud-bigtable.admin.table")
            }
            Usage::CloudBilling => String::from("https://www.googleapis.com/auth/cloud-billing"),
            Usage::CloudBillingReadOnly => {
                String::from("https://www.googleapis.com/auth/cloud-billing.readonly")
            }
            Usage::NDevCloudDnsReadOnly => {
                String::from("https://www.googleapis.com/auth/ndev.clouddns.readonly")
            }
            Usage::NDevCloudDnsReadWrite => {
                String::from("https://www.googleapis.com/auth/ndev.clouddns.readwrite")
            }
            Usage::CloudDebugger => String::from("https://www.googleapis.com/auth/cloud_debugger"),
            Usage::NDevCloudMan => String::from("https://www.googleapis.com/auth/ndev.cloudman"),
            Usage::NDevCloudManReadOnly => {
                String::from("https://www.googleapis.com/auth/ndev.cloudman.readonly")
            }
            Usage::DataStore => String::from("https://www.googleapis.com/auth/datastore"),
            Usage::CloudIdentityDevicesLookup => {
                String::from("https://www.googleapis.com/auth/cloud-identity.devices.lookup")
            }
            Usage::CloudIdentityGroups => {
                String::from("https://www.googleapis.com/auth/cloud-identity.groups")
            }
            Usage::CloudIdentityGroupsReadOnly => {
                String::from("https://www.googleapis.com/auth/cloud-identity.groups.readonly")
            }
            Usage::CloudKms => String::from("https://www.googleapis.com/auth/cloudkms"),
            Usage::LoggingAdmin => String::from("https://www.googleapis.com/auth/logging.admin"),
            Usage::LoggingRead => String::from("https://www.googleapis.com/auth/logging.read"),
            Usage::LoggingWrite => String::from("https://www.googleapis.com/auth/logging.write"),
            Usage::Monitoring => String::from("https://www.googleapis.com/auth/monitoring"),
            Usage::MonitoringRead => {
                String::from("https://www.googleapis.com/auth/monitoring.read")
            }
            Usage::MonitoringWrite => {
                String::from("https://www.googleapis.com/auth/monitoring.write")
            }
            Usage::CloudLanguage => String::from("https://www.googleapis.com/auth/cloud-language"),
            Usage::Compute => String::from("https://www.googleapis.com/auth/compute"),
            Usage::ComputeReadOnly => {
                String::from("https://www.googleapis.com/auth/compute.readonly")
            }
            Usage::PubSub => String::from("https://www.googleapis.com/auth/pubsub"),
            Usage::CloudRuntimeConfig => {
                String::from("https://www.googleapis.com/auth/cloudruntimeconfig")
            }
            Usage::SqlServiceAdmin => {
                String::from("https://www.googleapis.com/auth/sqlservice.admin")
            }
            Usage::CloudSearch => String::from("https://www.googleapis.com/auth/cloud_search"),
            Usage::CloudSearchDebug => {
                String::from("https://www.googleapis.com/auth/cloud_search.debug")
            }
            Usage::CloudSearchIndexing => {
                String::from("https://www.googleapis.com/auth/cloud_search.indexing")
            }
            Usage::CloudSearchQuery => {
                String::from("https://www.googleapis.com/auth/cloud_search.query")
            }
            Usage::CloudSearchSettings => {
                String::from("https://www.googleapis.com/auth/cloud_search.settings")
            }
            Usage::CloudSearchSettingsIndexing => {
                String::from("https://www.googleapis.com/auth/cloud_search.settings.indexing")
            }
            Usage::CloudSearchSettingsQuery => {
                String::from("https://www.googleapis.com/auth/cloud_search.settings.query")
            }
            Usage::CloudSearchStats => {
                String::from("https://www.googleapis.com/auth/cloud_search.stats")
            }
            Usage::CloudSearchStatsIndexing => {
                String::from("https://www.googleapis.com/auth/cloud_search.stats.indexing")
            }
            Usage::SourceFullControl => {
                String::from("https://www.googleapis.com/auth/source.full_control")
            }
            Usage::SourceReadOnly => {
                String::from("https://www.googleapis.com/auth/source.read_only")
            }
            Usage::SourceReadWrite => {
                String::from("https://www.googleapis.com/auth/source.read_write")
            }
            Usage::SpannerAdmin => String::from("https://www.googleapis.com/auth/spanner.admin"),
            Usage::SpannerData => String::from("https://www.googleapis.com/auth/spanner.data"),
            Usage::TraceAppend => String::from("https://www.googleapis.com/auth/trace.append"),
            Usage::CloudTranslation => {
                String::from("https://www.googleapis.com/auth/cloud-translation")
            }
            Usage::CloudVision => String::from("https://www.googleapis.com/auth/cloud-vision"),
            Usage::Content => String::from("https://www.googleapis.com/auth/content"),
            Usage::DriveActivity => String::from("https://www.googleapis.com/auth/drive.activity"),
            Usage::DriveActivityReadOnly => {
                String::from("https://www.googleapis.com/auth/drive.activity.readonly")
            }
            Usage::AppsLicensing => String::from("https://www.googleapis.com/auth/apps.licensing"),
            Usage::FirebaseMessaging => {
                String::from("https://www.googleapis.com/auth/firebase.messaging")
            }
            Usage::Firebase => String::from("https://www.googleapis.com/auth/firebase"),
            Usage::FirebaseReadOnly => {
                String::from("https://www.googleapis.com/auth/firebase.readonly")
            }
            Usage::FitnessActivityRead => {
                String::from("https://www.googleapis.com/auth/fitness.activity.read")
            }
            Usage::FitnessActivityWrite => {
                String::from("https://www.googleapis.com/auth/fitness.activity.write")
            }
            Usage::FitnessBloodGlucoseRead => {
                String::from("https://www.googleapis.com/auth/fitness.blood_glucose.read")
            }
            Usage::FitnessBloodGlucoseWrite => {
                String::from("https://www.googleapis.com/auth/fitness.blood_glucose.write")
            }
            Usage::FitnessBloodPressureRead => {
                String::from("https://www.googleapis.com/auth/fitness.blood_pressure.read")
            }
            Usage::FitnessBloodPressureWrite => {
                String::from("https://www.googleapis.com/auth/fitness.blood_pressure.write")
            }
            Usage::FitnessBodyRead => {
                String::from("https://www.googleapis.com/auth/fitness.body.read")
            }
            Usage::FitnessBodyWrite => {
                String::from("https://www.googleapis.com/auth/fitness.body.write")
            }
            Usage::FitnessBodyTemperatureRead => {
                String::from("https://www.googleapis.com/auth/fitness.body_temperature.read")
            }
            Usage::FitnessBodyTemperatureWrite => {
                String::from("https://www.googleapis.com/auth/fitness.body_temperature.write")
            }
            Usage::FitnessHeartRateRead => {
                String::from("https://www.googleapis.com/auth/fitness.heart_rate.read")
            }
            Usage::FitnessHeartRateWrite => {
                String::from("https://www.googleapis.com/auth/fitness.heart_rate.write")
            }
            Usage::FitnessLocationRead => {
                String::from("https://www.googleapis.com/auth/fitness.location.read")
            }
            Usage::FitnessLocationWrite => {
                String::from("https://www.googleapis.com/auth/fitness.location.write")
            }
            Usage::FitnessNutritionRead => {
                String::from("https://www.googleapis.com/auth/fitness.nutrition.read")
            }
            Usage::FitnessNutritionWrite => {
                String::from("https://www.googleapis.com/auth/fitness.nutrition.write")
            }
            Usage::FitnessOxygenSaturationRead => {
                String::from("https://www.googleapis.com/auth/fitness.oxygen_saturation.read")
            }
            Usage::FitnessOxygenSaturationWrite => {
                String::from("https://www.googleapis.com/auth/fitness.oxygen_saturation.write")
            }
            Usage::FitnessReproductiveHealthRead => {
                String::from("https://www.googleapis.com/auth/fitness.reproductive_health.read")
            }
            Usage::FitnessReproductiveHealthWrite => {
                String::from("https://www.googleapis.com/auth/fitness.reproductive_health.write")
            }
            Usage::FitnessSleepRead => {
                String::from("https://www.googleapis.com/auth/fitness.sleep.read")
            }
            Usage::FitnessSleepWrite => {
                String::from("https://www.googleapis.com/auth/fitness.sleep.write")
            }
            Usage::Genomics => String::from("https://www.googleapis.com/auth/genomics"),
            Usage::MailAddonsCurrentActionCompose => {
                String::from("https://www.googleapis.com/auth/gmail.addons.current.action.compose")
            }
            Usage::MailAddonsCurrentMessageAction => {
                String::from("https://www.googleapis.com/auth/gmail.addons.current.message.action")
            }
            Usage::MailAddonsCurrentMessageMetaData => String::from(
                "https://www.googleapis.com/auth/gmail.addons.current.message.metadata",
            ),
            Usage::MailAddonsCurrentMessageReadOnly => String::from(
                "https://www.googleapis.com/auth/gmail.addons.current.message.readonly",
            ),
            Usage::MailCompose => String::from("https://www.googleapis.com/auth/gmail.compose"),
            Usage::MailInsert => String::from("https://www.googleapis.com/auth/gmail.insert"),
            Usage::MailLabels => String::from("https://www.googleapis.com/auth/gmail.labels"),
            Usage::MailMetaData => String::from("https://www.googleapis.com/auth/gmail.metadata"),
            Usage::MailModify => String::from("https://www.googleapis.com/auth/gmail.modify"),
            Usage::MailReadOnly => String::from("https://www.googleapis.com/auth/gmail.readonly"),
            Usage::MailSend => String::from("https://www.googleapis.com/auth/gmail.send"),
            Usage::MailSettingsBasic => {
                String::from("https://www.googleapis.com/auth/gmail.settings.basic")
            }
            Usage::MailSettingsSharing => {
                String::from("https://www.googleapis.com/auth/gmail.settings.sharing")
            }
            Usage::AnalyticsEdit => String::from("https://www.googleapis.com/auth/analytics.edit"),
            Usage::AnalyticsManageUsers => {
                String::from("https://www.googleapis.com/auth/analytics.manage.users")
            }
            Usage::AnalyticsManageUsersReadOnly => {
                String::from("https://www.googleapis.com/auth/analytics.manage.users.readonly")
            }
            Usage::AnalyticsProVision => {
                String::from("https://www.googleapis.com/auth/analytics.provision")
            }
            Usage::AnalyticsUserDeletion => {
                String::from("https://www.googleapis.com/auth/analytics.user.deletion")
            }
            Usage::ChatDelete => String::from("https://www.googleapis.com/auth/chat.delete"),
            Usage::ChatMemberships => {
                String::from("https://www.googleapis.com/auth/chat.memberships")
            }
            Usage::ChatMembershipsApp => {
                String::from("https://www.googleapis.com/auth/chat.memberships.app")
            }
            Usage::ChatMembershipsReadOnly => {
                String::from("https://www.googleapis.com/auth/chat.memberships.readonly")
            }
            Usage::ChatMessages => String::from("https://www.googleapis.com/auth/chat.messages"),
            Usage::ChatMessagesCreate => {
                String::from("https://www.googleapis.com/auth/chat.messages.create")
            }
            Usage::ChatMessagesReactions => {
                String::from("https://www.googleapis.com/auth/chat.messages.reactions")
            }
            Usage::ChatMessagesReactionsCreate => {
                String::from("https://www.googleapis.com/auth/chat.messages.reactions.create")
            }
            Usage::ChatMessagesReactionsReadOnly => {
                String::from("https://www.googleapis.com/auth/chat.messages.reactions.readonly")
            }
            Usage::ChatMessagesReadOnly => {
                String::from("https://www.googleapis.com/auth/chat.messages.readonly")
            }
            Usage::ChatSpaces => String::from("https://www.googleapis.com/auth/chat.spaces"),
            Usage::ChatSpacesCreate => {
                String::from("https://www.googleapis.com/auth/chat.spaces.create")
            }
            Usage::ChatSpacesReadOnly => {
                String::from("https://www.googleapis.com/auth/chat.spaces.readonly")
            }
            Usage::ClassroomAnnouncements => {
                String::from("https://www.googleapis.com/auth/classroom.announcements")
            }
            Usage::ClassroomAnnouncementsReadOnly => {
                String::from("https://www.googleapis.com/auth/classroom.announcements.readonly")
            }
            Usage::ClassroomCourses => {
                String::from("https://www.googleapis.com/auth/classroom.courses")
            }
            Usage::ClassroomCoursesReadOnly => {
                String::from("https://www.googleapis.com/auth/classroom.courses.readonly")
            }
            Usage::ClassroomCourseworkMe => {
                String::from("https://www.googleapis.com/auth/classroom.coursework.me")
            }
            Usage::ClassroomCourseworkMeReadOnly => {
                String::from("https://www.googleapis.com/auth/classroom.coursework.me.readonly")
            }
            Usage::ClassroomCourseworkStudents => {
                String::from("https://www.googleapis.com/auth/classroom.coursework.students")
            }
            Usage::ClassroomCourseworkStudentsReadOnly => String::from(
                "https://www.googleapis.com/auth/classroom.coursework.students.readonly",
            ),
            Usage::ClassroomCourseworkMaterials => {
                String::from("https://www.googleapis.com/auth/classroom.courseworkmaterials")
            }
            Usage::ClassroomCourseworkMaterialsReadOnly => String::from(
                "https://www.googleapis.com/auth/classroom.courseworkmaterials.readonly",
            ),
            Usage::ClassroomGuardianLinksMeReadOnly => {
                String::from("https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly")
            }
            Usage::ClassroomGuardianLinksStudents => {
                String::from("https://www.googleapis.com/auth/classroom.guardianlinks.students")
            }
            Usage::ClassroomGuardianLinksStudentsReadOnly => String::from(
                "https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly",
            ),
            Usage::ClassroomProfileEmails => {
                String::from("https://www.googleapis.com/auth/classroom.profile.emails")
            }
            Usage::ClassroomProfilePhotos => {
                String::from("https://www.googleapis.com/auth/classroom.profile.photos")
            }
            Usage::ClassroomPushNotifications => {
                String::from("https://www.googleapis.com/auth/classroom.push-notifications")
            }
            Usage::ClassroomRosters => {
                String::from("https://www.googleapis.com/auth/classroom.rosters")
            }
            Usage::ClassroomRostersReadOnly => {
                String::from("https://www.googleapis.com/auth/classroom.rosters.readonly")
            }
            Usage::ClassroomStudentSubmissionsMeReadOnly => String::from(
                "https://www.googleapis.com/auth/classroom.student-submissions.me.readonly",
            ),
            Usage::ClassroomStudentSubmissionsStudentsReadOnly => String::from(
                "https://www.googleapis.com/auth/classroom.student-submissions.students.readonly",
            ),
            Usage::ClassroomTopics => {
                String::from("https://www.googleapis.com/auth/classroom.topics")
            }
            Usage::ClassroomTopicsReadOnly => {
                String::from("https://www.googleapis.com/auth/classroom.topics.readonly")
            }
            Usage::DocumentsReadOnly => {
                String::from("https://www.googleapis.com/auth/documents.readonly")
            }
            Usage::DriveMetaData => String::from("https://www.googleapis.com/auth/drive.metadata"),
            Usage::DriveMetaDataReadOnly => {
                String::from("https://www.googleapis.com/auth/drive.metadata.readonly")
            }
            Usage::DrivePhotosReadOnly => {
                String::from("https://www.googleapis.com/auth/drive.photos.readonly")
            }
            Usage::DriveScripts => String::from("https://www.googleapis.com/auth/drive.scripts"),
            Usage::AndroidEnterprise => {
                String::from("https://www.googleapis.com/auth/androidenterprise")
            }
            Usage::DriveAppData => String::from("https://www.googleapis.com/auth/drive.appdata"),
            Usage::Games => String::from("https://www.googleapis.com/auth/games"),
            Usage::AndroidPublisher => {
                String::from("https://www.googleapis.com/auth/androidpublisher")
            }
            Usage::Webmasters => String::from("https://www.googleapis.com/auth/webmasters"),
            Usage::WebmastersReadOnly => {
                String::from("https://www.googleapis.com/auth/webmasters.readonly")
            }
            Usage::SiteVerification => {
                String::from("https://www.googleapis.com/auth/siteverification")
            }
            Usage::SiteVerificationVerifyOnly => {
                String::from("https://www.googleapis.com/auth/siteverification.verify_only")
            }
            Usage::DriveFile => String::from("https://www.googleapis.com/auth/drive.file"),
            Usage::DriveReadOnly => String::from("https://www.googleapis.com/auth/drive.readonly"),
            Usage::Presentations => String::from("https://www.googleapis.com/auth/presentations"),
            Usage::PresentationsReadOnly => {
                String::from("https://www.googleapis.com/auth/presentations.readonly")
            }
            Usage::SpreadsheetsReadOnly => {
                String::from("https://www.googleapis.com/auth/spreadsheets.readonly")
            }
            Usage::Tasks => String::from("https://www.googleapis.com/auth/tasks"),
            Usage::TasksReadOnly => String::from("https://www.googleapis.com/auth/tasks.readonly"),
            Usage::EDiscovery => String::from("https://www.googleapis.com/auth/ediscovery"),
            Usage::EDiscoveryReadOnly => {
                String::from("https://www.googleapis.com/auth/ediscovery.readonly")
            }
            Usage::AppsAlerts => String::from("https://www.googleapis.com/auth/apps.alerts"),
            Usage::AppsOrder => String::from("https://www.googleapis.com/auth/apps.order"),
            Usage::AppsOrderReadOnly => {
                String::from("https://www.googleapis.com/auth/apps.order.readonly")
            }
            Usage::AppsGroupsMigration => {
                String::from("https://www.googleapis.com/auth/apps.groups.migration")
            }
            Usage::AppsGroupsSettings => {
                String::from("https://www.googleapis.com/auth/apps.groups.settings")
            }
            Usage::Indexing => String::from("https://www.googleapis.com/auth/indexing"),
            Usage::ManufacturerCenter => {
                String::from("https://www.googleapis.com/auth/manufacturercenter")
            }
            Usage::ContactsOtherReadOnly => {
                String::from("https://www.googleapis.com/auth/contacts.other.readonly")
            }
            Usage::ContactsReadOnly => {
                String::from("https://www.googleapis.com/auth/contacts.readonly")
            }
            Usage::DirectoryReadOnly => {
                String::from("https://www.googleapis.com/auth/directory.readonly")
            }
            Usage::UserAddressesRead => {
                String::from("https://www.googleapis.com/auth/user.addresses.read")
            }
            Usage::UserBirthdayRead => {
                String::from("https://www.googleapis.com/auth/user.birthday.read")
            }
            Usage::UserEmailsRead => {
                String::from("https://www.googleapis.com/auth/user.emails.read")
            }
            Usage::UserGenderRead => {
                String::from("https://www.googleapis.com/auth/user.gender.read")
            }
            Usage::UserOrganizationRead => {
                String::from("https://www.googleapis.com/auth/user.organization.read")
            }
            Usage::UserPhoneNumbersRead => {
                String::from("https://www.googleapis.com/auth/user.phonenumbers.read")
            }
            Usage::UserInfoProfile => {
                String::from("https://www.googleapis.com/auth/userinfo.profile")
            }
            Usage::PhotosLibrary => String::from("https://www.googleapis.com/auth/photoslibrary"),
            Usage::PhotosLibraryAppendOnly => {
                String::from("https://www.googleapis.com/auth/photoslibrary.appendonly")
            }
            Usage::PhotosLibraryEditAppCreatedData => {
                String::from("https://www.googleapis.com/auth/photoslibrary.edit.appcreateddata")
            }
            Usage::PhotosLibraryReadOnly => {
                String::from("https://www.googleapis.com/auth/photoslibrary.readonly")
            }
            Usage::PhotosLibraryReadOnlyAppCreatedData => String::from(
                "https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata",
            ),
            Usage::PhotosLibrarySharing => {
                String::from("https://www.googleapis.com/auth/photoslibrary.sharing")
            }
            Usage::SasPortal => String::from("https://www.googleapis.com/auth/sasportal"),
            Usage::DoubleClickSearch => {
                String::from("https://www.googleapis.com/auth/doubleclicksearch")
            }
            Usage::ServiceManagementReadOnly => {
                String::from("https://www.googleapis.com/auth/service.management.readonly")
            }
            Usage::ServiceManagement => {
                String::from("https://www.googleapis.com/auth/service.management")
            }
            Usage::StreetViewPublish => {
                String::from("https://www.googleapis.com/auth/streetviewpublish")
            }
            Usage::TagManagerDeleteContainers => {
                String::from("https://www.googleapis.com/auth/tagmanager.delete.containers")
            }
            Usage::TagManagerEditContainers => {
                String::from("https://www.googleapis.com/auth/tagmanager.edit.containers")
            }
            Usage::TagManagerEditContainerVersions => {
                String::from("https://www.googleapis.com/auth/tagmanager.edit.containerversions")
            }
            Usage::TagManagerManageAccounts => {
                String::from("https://www.googleapis.com/auth/tagmanager.manage.accounts")
            }
            Usage::TagManagerManageUsers => {
                String::from("https://www.googleapis.com/auth/tagmanager.manage.users")
            }
            Usage::TagManagerPublish => {
                String::from("https://www.googleapis.com/auth/tagmanager.publish")
            }
            Usage::TagManagerReadOnly => {
                String::from("https://www.googleapis.com/auth/tagmanager.readonly")
            }
            Usage::Youtube => String::from("https://www.googleapis.com/auth/youtube"),
            Usage::YoutubeChannelMembershipsCreator => {
                String::from("https://www.googleapis.com/auth/youtube.channel-memberships.creator")
            }
            Usage::YouTubeForceSsl => {
                String::from("https://www.googleapis.com/auth/youtube.force-ssl")
            }
            Usage::YoutubeReadOnly => {
                String::from("https://www.googleapis.com/auth/youtube.readonly")
            }
            Usage::YoutubeUpload => String::from("https://www.googleapis.com/auth/youtube.upload"),
            Usage::YoutubePartner => String::from("https://www.googleapis.com/auth/youtubepartner"),
            Usage::YoutubePartnerChannelAudit => {
                String::from("https://www.googleapis.com/auth/youtubepartner-channel-audit")
            }
            Usage::YoutubeAnalyticsMonetaryReadOnly => {
                String::from("https://www.googleapis.com/auth/yt-analytics-monetary.readonly")
            }
            Usage::YoutubeAnalyticsReadOnly => {
                String::from("https://www.googleapis.com/auth/yt-analytics.readonly")
            }
            Usage::Email => String::from("email"),
            Usage::OpenId => String::from("openid"),
            Usage::Profile => String::from("profile"),
        }
    }
}
