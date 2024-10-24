import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('CSR_LM/CSRLM_Login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/CSR_Customer_Employer/Page_Loan Manager - LoanDashboard/span_Customer Info'))

WebUI.executeJavaScript('document.body.style.zoom=\'80%\'', null)

WebUI.click(findTestObject('Object Repository/CSR_Customer_Employer/Page_Loan Manager - Customer/span_Employer Information'))

WebUI.click(findTestObject('Object Repository/CSR_Customer_Employer/Page_Loan Manager - Customer/i_RHF_fa-solid fa-square-pen ng-tns-c258-19'))

WebUI.click(findTestObject('Object Repository/CSR_Customer_Employer/Page_Loan Manager - Customer/td_employer name test'))

WebUI.setText(findTestObject('Object Repository/CSR_Customer_Employer/Page_Loan Manager - Customer/input_Info Last Verified com_id_inp_emprInf_f83241'), 
    'Ram')

WebUI.click(findTestObject('Object Repository/CSR_Customer_Employer/Page_Loan Manager - Customer/div_B - A DOUGLAS BLACK'))

WebUI.click(findTestObject('Object Repository/CSR_Customer_Employer/Page_Loan Manager - Customer/span_B - A DOUGLAS BLACK'))

WebUI.click(findTestObject('Object Repository/CSR_Customer_Employer/Page_Loan Manager - Customer/td_Edit DetailsAll fields with an (asterisk_22589b'))

WebUI.setText(findTestObject('Object Repository/CSR_Customer_Employer/Page_Loan Manager - Customer/input_BorrowerCo-Borrower_id_inp_emprInfo_H_cb0cdc'), 
    '(222)222-2222')

